#!/usr/bin/env python3
import asyncio
import json
import logging
from typing import Any, Optional, List, Callable, cast, TYPE_CHECKING

import voluptuous
from aiohttp import web
from aiohttp.typedefs import LooseHeaders, JSONEncoder
from aiohttp.web_exceptions import (
    HTTPInternalServerError, HTTPBadRequest, HTTPUnauthorized,
)

from core.context import Context
if TYPE_CHECKING:
    from core.core import ApplicationCore
from core.webserver import exceptions
from core.webserver.status import HTTP_OK, HTTP_SERVICE_UNAVAILABLE
from core.webserver.type import APPLICATION_JSON
_LOGGER = logging.getLogger(__name__)
KEY_AUTHENTICATED = "authenticated"


def is_callback(func: Callable[..., Any]) -> bool:
    """Check if function is safe to be called in the event loop."""
    return getattr(func, "_callback", False) is True


class RequestView:
    """Base request."""

    requires_auth = True
    url: Optional[str] = None

    extra_urls: List[str] = []

    @staticmethod
    def context(request: web.Request) -> Context:
        """Generate a context from a request."""
        user = request.get("hass_user")
        if user is None:
            return Context()

        return Context(user_id=user.id)

    @staticmethod
    def json(
        result: Any,
        status_code: int = HTTP_OK,
        headers: Optional[LooseHeaders] = None,
    ) -> web.Response:
        """Return a JSON response."""
        try:
            msg = json.dumps(result, cls=ComplexEncoder, allow_nan=False).encode("UTF-8")
        except (ValueError, TypeError) as err:
            _LOGGER.error(f"Unable to serialize to JSON: {err}\n{result}")
            raise HTTPInternalServerError from err
        response = web.Response(
            body=msg,
            content_type=APPLICATION_JSON,
            status=status_code,
            headers=headers,
        )
        response.enable_compression()
        return response

    def json_message(
        self,
        message: str,
        status_code: int = HTTP_OK,
        message_code: Optional[str] = None,
        headers: Optional[LooseHeaders] = None,
    ) -> web.Response:
        """Return a JSON message response."""
        data = {"message": message}
        if message_code is not None:
            data["code"] = message_code
        return self.json(data, status_code, headers=headers)

    def register(self, core: "ApplicationCore", router: web.UrlDispatcher) -> None:
        """Register the view with a router."""
        assert self.url is not None, "No url set for view"
        urls = [self.url] + self.extra_urls
        routes = []

        for method in ("get", "post", "delete", "put", "patch", "head", "options"):
            handler = getattr(self, method, None)

            if not handler:
                continue

            handler = request_handler_factory(self, core, handler)

            for url in urls:
                routes.append(router.add_route(method, url, handler))


class ComplexEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, complex):
            return [obj.real, obj.imag]
        # Let the base class default method raise the TypeError
        return json.JSONEncoder.default(self, obj)


def request_handler_factory(view: RequestView, core: "ApplicationCore", handler: Callable) -> Callable:
    """Wrap the handler classes."""
    assert asyncio.iscoroutinefunction(handler) or is_callback(
        handler
    ), "Handler should be a coroutine or a callback."

    async def handle(request: web.Request) -> web.StreamResponse:
        """Handle incoming request."""
        if core.is_stopping:
            return web.Response(status=HTTP_SERVICE_UNAVAILABLE)

        authenticated = request.get(KEY_AUTHENTICATED, False)

        if view.requires_auth and not authenticated:
            raise HTTPUnauthorized()

        _LOGGER.debug(f"Serving {request.path} to {request.remote} (auth: {authenticated})")

        try:
            result = handler(view, request, **request.match_info)

            if asyncio.iscoroutine(result):
                result = await result
        except voluptuous.Invalid as err:
            raise HTTPBadRequest() from err
        except exceptions.ServiceNotFound as err:
            raise HTTPInternalServerError() from err
        except exceptions.Unauthorized as err:
            raise HTTPUnauthorized() from err

        if isinstance(result, web.StreamResponse):
            # The method handler returned a ready-made Response, how nice of it
            return result

        status_code = HTTP_OK

        if isinstance(result, tuple):
            result, status_code = result

        if isinstance(result, bytes):
            bresult = result
        elif isinstance(result, str):
            bresult = result.encode("utf-8")
        elif result is None:
            bresult = b""
        else:
            assert (
                False
            ), f"Result should be None, string, bytes or Response. Got: {result}"

        return web.Response(body=bresult, status=status_code)

    return handle
