"""HTTP server implementation."""
import base64
import binascii
import logging
from ipaddress import ip_address
from typing import TYPE_CHECKING

import aiohttp_jinja2
import jinja2
from aiohttp import hdrs, web
from aiohttp.web_exceptions import HTTPForbidden, HTTPUnauthorized
from aiohttp.web_middlewares import middleware
from oauth2 import Provider
from oauth2.error import UserNotAuthenticated
from oauth2.grant import AuthorizationCodeGrant
from oauth2.store.memory import ClientStore, TokenStore
from oauth2.tokengenerator import Uuid4TokenGenerator
from oauth2.web import AuthorizationCodeGrantSiteAdapter
from oauth2.web.aiohttp import OAuth2Handler

from .. import const
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

        # initialize oauth-stateless
        class TestSiteAdapter(AuthorizationCodeGrantSiteAdapter):

            CONFIRMATION_TEMPLATE = """
            <html>
                <body>
                    <p>
                        <a href="{url}&confirm=1">confirm</a>
                    </p>
                    <p>
                        <a href="{url}&confirm=0">deny</a>
                    </p>
                </body>
            </html>
                """

            def render_auth_page(self, request, response, environ, scopes, client):
                _LOGGER.debug("render_auth_page()")
                page_url = request.path + "?" + request.query_string
                response.body = self.CONFIRMATION_TEMPLATE.format(url=page_url)

                return response

            def authenticate(self, request, environ, scopes, client):
                _LOGGER.debug("authenticate()")
                example_user_id = 123
                example_ext_data = {}
                if request.method == "GET":
                    if request.get_param("confirm") == "1":
                        return example_ext_data, example_user_id
                raise UserNotAuthenticated

            def user_has_denied_access(self, request):
                _LOGGER.debug("user_has_denied_access()")
                if request.method == "GET":
                    if request.get_param("confirm") == "0":
                        return True
                return False

        client_store = ClientStore()
        client_store.add_client(
            client_id="abc",
            client_secret="xyz",
            redirect_uris=[
                "http://127.0.0.1:8000/callback",
                "https://oauthdebugger.com/debug",
            ],
        )
        token_store = TokenStore()

        provider = Provider(
            access_token_store=token_store,
            auth_code_store=token_store,
            client_store=client_store,
            token_generator=Uuid4TokenGenerator(),
        )
        provider.add_grant(AuthorizationCodeGrant(site_adapter=TestSiteAdapter()))
        oauth_handler = OAuth2Handler(provider)
        self.app.router.add_get(provider.authorize_path, oauth_handler.dispatch_request)
        self.app.router.add_post(
            provider.authorize_path, oauth_handler.post_dispatch_request
        )
        self.app.router.add_post(
            provider.token_path, oauth_handler.post_dispatch_request
        )

        self.app.middlewares.append(self.ban_middleware)
        self.app.middlewares.append(self.auth_middleware)
        self.app.middlewares.append(self.headers_middleware)
        # TODO: add cors middleware
        # self.app.middlewares.append(self.cors_middleware)

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
        await self.runner.setup()

        host = self.core.config.external_url
        port = self.core.config.port
        _LOGGER.debug(f"ignore host from config {host}")

        # use host=None to listen on all interfaces.
        site = web.TCPSite(runner=self.runner, host=None, port=port)
        _LOGGER.info(f"Webserver is listening on {site._host}:{site._port}")
        await site.start()

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
                msg = f"{msg} ({user_agent})"

            _LOGGER.warning(msg)

            # track failed login attempt
            if remote_addr in self.core.failed_logins:
                # check login attempt count
                if self.core.failed_logins[remote_addr] >= LOGIN_ATTEMPT_THRESHOLD:
                    _LOGGER.warning(
                        f"Banned IP {remote_addr} for too many login attempts"
                    )
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
                auth_type, auth_val = request.headers.get(hdrs.AUTHORIZATION).split(
                    " ", 1
                )
            except ValueError:
                # If no space in authorization header
                return False

            if auth_type != "Bearer":
                return False

            # TODO: validate authorization credentials
            _LOGGER.error(f"validate authorization credentials: {auth_val}")

            try:
                authenticated = True
                auth_type = "bearer token"
                base64_bytes = auth_val.encode("ascii")
                message_bytes = base64.b64decode(base64_bytes)
                token = message_bytes.decode("ascii")
                user_id = token.split(":", 1)[0]
                assert len(user_id) > 1
            except (binascii.Error, AssertionError):
                _LOGGER.error("Failed to extract user from token!")
                user_id = None
                authenticated = False

            if authenticated:
                _LOGGER.debug(
                    f"Authenticated {request.remote} for {request.path} using {auth_type}. User = {user_id}"
                )
                request[KEY_AUTHENTICATED] = authenticated
                request[KEY_USER_ID] = user_id

        return await handler(request)

    @middleware
    async def headers_middleware(self, request: web.Request, handler):
        """Add a server header to all responses."""
        response = await handler(request)
        if response is not None:
            response.headers["Server"] = f"Photos.network/{const.CORE_VERSION}"
        return response
