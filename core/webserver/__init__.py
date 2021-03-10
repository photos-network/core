"""HTTP server implementation."""
import logging
from ipaddress import ip_address
from typing import TYPE_CHECKING, Optional

import aiohttp_jinja2
import jinja2
import jwt
from aiohttp import hdrs, web
from aiohttp.web_exceptions import HTTPForbidden, HTTPUnauthorized
from aiohttp.web_middlewares import middleware

from .. import const
from ..authentication import Auth, AuthClient
from ..authentication.auth_database import AuthDatabase
from .request import KEY_AUTHENTICATED, KEY_USER_ID, RequestView  # noqa: F401

if TYPE_CHECKING:
    from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
MAX_CLIENT_SIZE: int = 1024 ** 2 * 16
LOGIN_ATTEMPT_THRESHOLD = 3


class Webserver:
    """Webserver implementation."""

    def __init__(self, core: "ApplicationCore"):
        """Initialize webserver."""
        self.core = core
        self.app = web.Application(middlewares=[], client_max_size=MAX_CLIENT_SIZE)
        self.runner = web.AppRunner(self.app)

        # init jinja2 template engine
        aiohttp_jinja2.setup(
            self.app, loader=jinja2.FileSystemLoader("core/webserver/templates")
        )

        self.app.middlewares.append(self.ban_middleware)
        self.app.middlewares.append(self.auth_middleware)
        self.app.middlewares.append(self.headers_middleware)

    def register_request(self, view):
        """
        Register a request.

        The view argument must be a class that inherits from Request.
        It is optional to instantiate it before registering; this method will
        handle it either way.
        """
        # if isinstance(view, type):
        #     # Instantiate the view, if needed
        #     view = view()

        if not hasattr(view, "url"):
            class_name = view.__class__.__name__
            raise AttributeError(f'{class_name} missing required attribute "url"')

        if not hasattr(view, "name"):
            class_name = view.__class__.__name__
            raise AttributeError(f'{class_name} missing required attribute "name"')

        view.register(self.core, self.app.router)

    async def start(self):
        """Start webserver."""
        await self.init_auth()

        await self.runner.setup()

        host = self.core.config.external_url
        port = self.core.config.port
        _LOGGER.debug(f"ignore host from config {host}")

        # use host=None to listen on all interfaces.
        site = web.TCPSite(runner=self.runner, host=None, port=port)
        await site.start()
        _LOGGER.info(f"Webserver is listening on {site._host}:{site._port}")

    async def init_auth(self):
        database_file = f"{self.core.config.data_dir}/system.sqlite3"
        auth_database = AuthDatabase(database_file)

        # setup auth
        self.auth = Auth(self.app, auth_database)

        # TODO: read client config from configuration
        self.auth.add_client(
            AuthClient(
                client_name="Frontend",
                client_id="d37c098d-ac25-4a96-b462-c1ca05f45952",
                client_secret="AYgD5Y2DV7bbWupYW7WmYQ",
                redirect_uris=[
                    "http://127.0.0.1:3000/callback",
                ],
            )
        )
        self.auth.add_client(
            AuthClient(
                client_name="Android App",
                client_id="1803463f-c10f-4a65-aa15-b2e39be9f14d",
                client_secret="1TmlFYywRd7MwlbRNiePjQ",
                redirect_uris=["photosapp://authenticate"],
            )
        )

        _LOGGER.info("init_auth done")

    async def get_user_id(self, request: web.Request) -> Optional[str]:
        return await self.auth.check_authorized(request)

    async def stop(self):
        """Stop webserver."""
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
                msg = f"{msg} [{user_agent}]"

            _LOGGER.warning(msg)

            # track failed login attempt
            if str(remote_addr) in self.core.failed_logins:
                count = self.core.failed_logins[str(remote_addr)]

                # check login attempt count
                if count >= LOGIN_ATTEMPT_THRESHOLD:
                    _LOGGER.error(
                        f"Banned IP {remote_addr} for too many login attempts"
                    )
                    self.core.banned_ips.append(str(remote_addr))
                else:
                    _LOGGER.warning(
                        f"{count + 1}. failed login attempts for IP {remote_addr}"
                    )
                    self.core.failed_logins[str(remote_addr)] = count + 1
            else:
                _LOGGER.warning(f"1. failed login attempts for IP {remote_addr}")
                self.core.failed_logins[str(remote_addr)] = 1

            raise

    @middleware
    async def auth_middleware(self, request: web.Request, handler):
        """Check authentication for requests."""
        authenticated = False

        if hdrs.AUTHORIZATION in request.headers:
            try:
                auth_type, auth_val = request.headers.get(hdrs.AUTHORIZATION).split(
                    " ", 1
                )
            except ValueError:
                # If no space in authorization header
                return False

            if auth_type != "Bearer":
                return False

            try:
                jwt.decode(auth_val, "secret", algorithms="HS256")
                authenticated = True

            except jwt.ExpiredSignatureError as e:
                authenticated = False
                _LOGGER.error(f"Could not verify token! {e}")

            if authenticated:
                _LOGGER.debug(f"Authenticated {request.remote} for {request.path}.")
                request[KEY_AUTHENTICATED] = authenticated

        return await handler(request)

    @middleware
    async def headers_middleware(self, request: web.Request, handler):
        """Add a server header to all responses."""
        response = await handler(request)
        if response is not None:
            response.headers["Server"] = f"Photos.network/{const.CORE_VERSION}"
        return response
