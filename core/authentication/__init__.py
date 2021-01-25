"""Initialization of authentication."""
import json
import logging
import secrets

import aiohttp_jinja2
from aiohttp import web
from authlib.oauth2 import AuthorizationServer, ClientAuthentication, OAuth2Request

from core.webserver.request import ComplexEncoder
from core.webserver.status import HTTP_OK
from core.webserver.type import APPLICATION_JSON

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Auth(AuthorizationServer):
    def __init__(self, application):
        self.app = application

        self.app.router.add_post(
            path="/login", handler=self.login_handler2, name="auth:login2"
        )
        self.app.router.add_get(
            path="/oauth/authorize",
            handler=self.authorization_handler,
            name="auth:authorize",
        )
        self.app.router.add_post(
            path="/oauth/authorize",
            handler=self.authorization_handler2,
            name="auth:authorize2",
        )
        self.app.router.add_post(
            path="/oauth/access_token",
            handler=self.access_handler,
            name="auth:access_token",
        )

    async def login_handler2(self, request):
        _LOGGER.warning("POST /login")
        response_type = "code"
        client_id = "ABCDEF123"
        redirect_uri = request.host
        scope = "user public_photos"
        state = secrets.token_hex(16)

        raise web.HTTPFound(
            location=f"http://127.0.0.1:9090/oauth/authorize?response_type={response_type}&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&state={state}"
        )

    @aiohttp_jinja2.template("tmpl.jinja2")
    async def authorization_handler(self, request):
        """Endpoint where the user get its authorization."""
        _LOGGER.warning("GET /oauth/access_token")
        _LOGGER.warning(f"request: {request.host}")

        response_type = request.query.get("response_type")
        client_id = request.query.get("client_id")
        redirect_uri = request.query.get("redirect_uri")
        scope = request.query.get("scope")
        state = request.query.get("state")

        _LOGGER.warning(f"a: {response_type}")
        _LOGGER.warning(f"b: {client_id}")
        _LOGGER.warning(f"c: {redirect_uri}")
        _LOGGER.warning(f"d: {scope}")
        _LOGGER.warning(f"e: {state}")

        _LOGGER.warning(f"request: {request.query_string}")
        # _LOGGER.warning(f"request: {request.__dict__}")

        return {"redirect_uri": redirect_uri}

    async def authorization_handler2(self, request):
        _LOGGER.warning("POST /oauth/access_token")
        data = await request.post()
        return web.Response()

    async def access_handler(self, request):
        """Endpoint to request an access token."""
        _LOGGER.warning("GET /oauth/access_token")

        success = True
        if success:
            msg = {
                "access_token": "MTQ0NjJkZmQ5OTM2NDE1ZTZjNGZmZjI3",
                "token_type": "bearer",
                "expires_in": 3600,
                "refresh_token": "IwOGYzYTlmM2YxOTQ5MGE3YmNmMDFkNTVk",
                "scope": "create",
            }
            response = web.Response(
                body=json.dumps(msg, cls=ComplexEncoder, allow_nan=False).encode(
                    "UTF-8"
                ),
                content_type=APPLICATION_JSON,
                status=HTTP_OK,
            )
            response.enable_compression()
            return response
        else:
            return web.Response()
