"""Initialization of authentication."""
import json
import logging
from typing import List

import aiohttp_jinja2
from aiohttp import web

from core.webserver.request import ComplexEncoder
from core.webserver.status import HTTP_OK
from core.webserver.type import APPLICATION_JSON

from .auth_client import AuthClient

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Auth:
    # list of registered auth clients
    auth_clients: List[AuthClient] = []

    # TODO: save redirect uri in session
    redirect_uri = None
    # TODO: save state uri in session
    state = None

    def __init__(self, application):
        self.app = application

        # Authorization Endpoint: obtain an authorization grant
        self.app.router.add_get(
            path="/oauth/authorize", handler=self.authorization_endpoint_get
        )
        self.app.router.add_post(
            path="/oauth/authorize", handler=self.authorization_endpoint_post
        )

        # Token Endpoint: obtain an access token by authorization grant or refresh token
        self.app.router.add_post(
            path="/oauth/token", handler=self.token_endpoint_handler
        )

        # self.app.router.add_post(path="/login", handler=self.login_handler2, name="auth:login2")
        # self.app.router.add_post("/login", self.login_handler, name="login")
        self.app.router.add_get("/logout", self.logout_handler, name="logout")
        # self.app.router.add_get("/public", self.internal_handler, name="public")
        self.app.router.add_get("/protected", self.protected_handler, name="protected")

    def add_client(self, auth_client: AuthClient):
        self.auth_clients.append(auth_client)

    async def logout_handler(self, request):
        _LOGGER.info("GET /logout")
        # await check_authorized(request)
        response = web.Response(body=b"You have been logged out")
        # await forget(request, response)
        return response

    async def protected_handler(self, request):
        _LOGGER.info("protected_handler")
        # await check_permission(request, "protected")
        response = web.Response(body=b"You are on protected page")
        return response

    @aiohttp_jinja2.template("authorize.jinja2")
    async def authorization_endpoint_get(self, request):
        """Endpoint where the user get its authorization."""
        try:
            _LOGGER.info(f"GET /oauth/authorize  from: {request.host}")
            _LOGGER.debug(f"request: {request.query}")

            response_type = request.query.get("response_type")
            client_id = request.query.get("client_id")

            # validate required params
            if response_type is None or client_id is None:
                _LOGGER.warning("The request is missing a required parameter")
                data = """{
                    "error": "invalid_request",
                    "error_description": "The request is missing a required parameter"
                    }"""
                return web.json_response(json.loads(data))

            # check if client is known
            if not any(client.client_id == client_id for client in self.auth_clients):
                _LOGGER.warning("The request is missing a required parameter")
                data = '{"error":"unauthorized_client"}'
                return web.json_response(json.loads(data))

            # validate response_type
            if response_type != "code":
                _LOGGER.warning(
                    f"The request is using an invalid response_type: {response_type}"
                )
                data = """{
                    "error":"unsupported_response_type",
                    "error_description":"The request is using an invalid response_type"
                    }"""
                return web.json_response(json.loads(data))

            redirect_uri = request.query.get("redirect_uri")

            # extract client from registered auth_clients with matching client_id
            registered_auth_client = next(
                filter(lambda client: client.client_id == client_id, self.auth_clients),
                None,
            )
            # validate if redirect_uri is in registered_auth_client
            if not any(
                uri == redirect_uri for uri in registered_auth_client.redirect_uris
            ):
                _LOGGER.error("redirect uri not found:")
                data = '{"error":"unauthorized_client"}'
                return web.json_response(json.loads(data))

            scope = request.query.get("scope")
            requested_scopes = scope.split(" ")

            # TODO: validate scopes with regex =>  1*( %x21 / %x23-5B / %x5D-7E ))
            registered_scopes = [
                "openid",
                "profile",
                "email",
                "phone",
                "library:read",
                "library:append",
                "library:edit",
                "library:write",
                "library:share",
                "admin.users:read",
                "admin.users:invite",
                "admin.users:write",
            ]
            _LOGGER.info(
                f"found {len(registered_scopes)} registered scopes and {len(requested_scopes)} requested scopes."
            )

            # check if the requested scope is registered
            for requested_scope in requested_scopes:
                if requested_scope not in registered_scopes:
                    _LOGGER.error(
                        f"The requested scope '{requested_scope}' is invalid, unknown, or malformed."
                    )
                    data = """{
                        "error":"invalid_scope",
                        "error_description":"The requested scope is invalid, unknown, or malformed."
                    }"""
                    return web.json_response(json.loads(data))

            # TODO: used for preventing cross-site request forgery
            # [Section 10.12](https://tools.ietf.org/html/rfc6749#section-10.12)
            self.state = request.query.get("state")

            _LOGGER.debug(f"response_type : {response_type}")
            _LOGGER.debug(f"client_id     : {client_id}")
            # _LOGGER.debug(f"client_secret : {client_secret}")
            _LOGGER.debug(f"redirect_uri  : {self.redirect_uri}")
            _LOGGER.debug(f"scope         : {scope}")
            _LOGGER.debug(f"state         : {self.state}")
            _LOGGER.debug(f"request       : {request.query_string}")

            return {
                "redirect_uri": self.redirect_uri,
                "requesting_app": "Frontend",
                "permissions": [
                    {
                        "scope": "openid",
                        "localized": "access the users public profile e.g.: username",
                    },
                    {
                        "scope": "profile",
                        "localized": "access the users personal profile information e.g.: firstname, lastname",
                    },
                    {
                        "scope": "email",
                        "localized": "access the users associated email address.",
                    },
                    {
                        "scope": "phone",
                        "localized": "access the users associated phone number.",
                    },
                    {
                        "scope": "library.read",
                        "localized": "Read only Grant the user to list all photos owned by the user.",
                    },
                    {
                        "scope": "library.append",
                        "localized": "Limited write access Grant the user to add new photos, create new albums.",
                    },
                    {
                        "scope": "library.edit",
                        "localized": "Grant the user to edit photos owned by the user.",
                    },
                    {
                        "scope": "library.write",
                        "localized": "Grant the user to add and edit photos, albums, tags.",
                    },
                    {
                        "scope": "library.share",
                        "localized": "Grant the user to create new shares (photos/videos/albums).",
                    },
                    {
                        "scope": "admin.users:read",
                        "localized": "Grant the user to list users on the system.",
                    },
                    {
                        "scope": "admin.users:invite",
                        "localized": "Grant the user to invite new users to the system.",
                    },
                    {
                        "scope": "admin.users:write",
                        "localized": "Grant the user to manage users on the system.",
                    },
                ],
            }
        except Exception as e:
            # This error code is needed because a 500 Internal Server
            # Error HTTP status code cannot be returned to the client
            # via an HTTP redirect.
            _LOGGER.error(f"an unexpected error happened: {e}")
            data = '{"error":"server_error"}'
            return web.json_response(json.loads(data))

    async def authorization_endpoint_post(self, request):
        _LOGGER.warning("POST /oauth/authorize")
        data = await request.post()

        username = data["uname"]
        password = data["password"]

        # TODO: check credentials
        _LOGGER.warning(f"username      : {username}")
        _LOGGER.warning(f"password      : {password}")

        # TODO: create an authorization code
        self.authorization_code = "SplxlOBeZQQYbYS6WxSbIA"

        _LOGGER.warning(f"redirect_uri  : {self.redirect_uri}")

        request_fails = False

        # TODO: return error_reason based on the error.
        error_reason = "access_denied"

        if request_fails:
            _LOGGER.warning(f"redirect with error {error_reason}")
            raise web.HTTPFound(
                f"{self.redirect_uri}?error={error_reason}&state={self.state}"
            )

    async def authorization_handler2(self, request):
        _LOGGER.warning("POST /oauth/access_token")
        data = await request.post()
        _LOGGER.warning(f"data: {data}")
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
            web.Response(
                body=json.dumps(msg, cls=ComplexEncoder, allow_nan=False).encode(
                    "UTF-8"
                ),
                content_type=APPLICATION_JSON,
                status=HTTP_OK,
            )
        else:
            _LOGGER.info(f"redirect with success {self.authorization_code}")
            _LOGGER.debug(
                f"{self.redirect_uri}?code={self.authorization_code}&state={self.state}"
            )
            raise web.HTTPFound(
                f"{self.redirect_uri}?code={self.authorization_code}&state={self.state}"
            )

    async def token_endpoint_handler(self, request):
        _LOGGER.warning("POST /oauth/token")

        data = await request.post()
        # grant_type is REQUIRED
        if "grant_type" not in data:
            data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))

        grant_type = data["grant_type"]
        _LOGGER.debug(f"grant_type    : {grant_type}")

        if grant_type == "authorization_code":
            return await self._handle_authorization_code_request(data)
        elif grant_type == "refresh_token":
            return await self._handle_refresh_token_request(data)

    async def _handle_authorization_code_request(self, data):
        """Handle authorization code requests."""
        state = None
        if "state" in data:
            state = data["state"]

        # authorization_code is REQUIRED
        if "code" not in data:
            if state:
                data = '{"error":"invalid_request", "state":"' + state + '"}'
            else:
                data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))

        # redirect_uri is REQUIRED
        if "redirect_uri" not in data:
            data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))
        redirect_uri = data["redirect_uri"]

        # Compare authorization_code with previous code
        code = data["code"]
        if self.authorization_code != code:
            data = '{"error":"invalid_request"}'
            _LOGGER.error("Could not validate given code!")
            return web.json_response(json.loads(data))

        # client_id is REQUIRED
        if "client_id" not in data:
            data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))
        client_id = data["client_id"]

        _LOGGER.debug(f"code          : {code}")
        _LOGGER.debug(f"redirect_uri  : {redirect_uri}")
        _LOGGER.debug(f"client_id     : {client_id}")

        data = """{
            "access_token": self._generate_acess_token(),
            "token_type":"example",
            "expires_in":3600,
            "refresh_token":"tGzv3JOkF0XG5Qx2TlKWIA",
            "example_parameter":"example_value"
        }"""
        return web.json_response(json.loads(data))

    async def _handle_refresh_token_request(self, data):
        """Handle refresh token requests."""
        refresh_token = data["refresh_token"]

        _LOGGER.debug(f"refresh_token : {refresh_token}")

        data = """{
            "access_token":"2YotnFZFEjr1zCsicMWpAA",
            "token_type":"example",
            "expires_in":3600,
            "refresh_token":"tGzv3JOkF0XG5Qx2TlKWIA",
            "example_parameter":"example_value"
        }"""
        return web.json_response(json.loads(data))

    def _generate_access_token() -> str:
        return "2YotnFZFEjr1zCsicMWpAA"
