"""HTTP server implementation."""
from ipaddress import ip_address
from typing import TYPE_CHECKING, TypeVar, Type

from aiohttp import web, hdrs
import logging

from aiohttp.web_exceptions import HTTPForbidden, HTTPUnauthorized
from aiohttp.web_middlewares import middleware

if TYPE_CHECKING:
    from core.webserver.request import Request
    from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
MAX_CLIENT_SIZE: int = 1024 ** 2 * 16
LOGIN_ATTEMPT_THRESHOLD = 3
KEY_AUTHENTICATED = "authenticated"


class Webserver:
    """Webserver implementation"""

    def __init__(self, core: "ApplicationCore"):
        self.core = core
        self.app = web.Application(
            middlewares=[],
            client_max_size=MAX_CLIENT_SIZE)
        self.runner = web.AppRunner(self.app)

        self.app.middlewares.append(self.ban_middleware)
        self.app.middlewares.append(self.auth_middleware)
        # TODO: add cors middleware
        # self.app.middlewares.append(self.cors_middleware)

    def register_request(self, view: Type["Request"]):
        """Register a request, wich must inherit from Request"""
        if not hasattr(view, "url"):
            class_name = view.__class__.__name__
            raise AttributeError(f'{class_name} missing required attribute "url"')

        if not hasattr(view, "name"):
            class_name = view.__class__.__name__
            raise AttributeError(f'{class_name} missing required attribute "name"')

        view.register(view, self.app, self.app.router)

    async def start(self):
        await self.runner.setup()

        site = web.TCPSite(
            runner=self.runner,
            host='localhost',
            port=8080)
        await site.start()

    async def stop(self):
        await self.runner.cleanup()

    @middleware
    async def ban_middleware(self, request: web.Request, handler):
        """Block IP if threshold of failed login attempts exceeds."""
        remote_ip = ip_address(request.remote)
        is_banned = remote_ip in self.core.banned_ips

        # return with Forbidden if already banned
        if is_banned:
            raise HTTPForbidden()

        try:
            return await handler(request)
        except HTTPUnauthorized:
            remote_addr = ip_address(request.remote)
            remote_host = request.remote

            msg = f"Login attempt or request with invalid authentication from {remote_host} ({remote_addr})"

            user_agent = request.headers.get("user-agent")
            if user_agent:
                msg = f"{msg} ({user_agent})"

            _LOGGER.warning(msg)

            # track failed login attempt
            if remote_addr in self.core.failed_logins:
                # check login attempt count
                if self.core.failed_logins[remote_addr] >= LOGIN_ATTEMPT_THRESHOLD:
                    _LOGGER.warning(f"Banned IP {remote_addr} for too many login attempts")
                    self.core.banned_ips.append(remote_addr)
                else:
                    old_count = self.core.banned_ips[remote_addr]
                    self.core.banned_ips[remote_addr] = old_count + 1
            else:
                self.core.failed_logins[remote_addr] = 1

            raise

    @middleware
    async def auth_middleware(self, request: web.Request, handler):
        """Check authentication for requests."""
        authenticated = False

        if hdrs.AUTHORIZATION in request.headers:
            try:
                auth_type, auth_val = request.headers.get(hdrs.AUTHORIZATION).split(" ", 1)
            except ValueError:
                # If no space in authorization header
                return False

            if auth_type != "Bearer":
                return False

            _LOGGER.error(f"validate auth_val: {auth_val}")

            authenticated = True
            auth_type = "bearer token"

        if authenticated:
            _LOGGER.debug(f"Authenticated {request.remote} for {request.path} using {auth_type}")
            request[KEY_AUTHENTICATED] = authenticated

        return await handler(request)
