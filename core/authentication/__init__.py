"""Initialization of authentication."""
import json
import logging

import aiohttp_jinja2
from aiohttp import web

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Auth:
    redirect_uri = None
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

    @aiohttp_jinja2.template("authorize.jinja2")
    async def authorization_endpoint_get(self, request):
        """Endpoint where the user get its authorization."""
        _LOGGER.warning("GET /oauth/authorize")
        _LOGGER.info(f"incoming auth request from {request.host}")
        _LOGGER.debug(f"request: {request.query}")

        # TODO: response_type MUST be set to "code"
        response_type = request.query.get("response_type")

        # TODO: check if client_id is known
        client_id = request.query.get("client_id")
        client_secret = request.query.get("client_secret")

        self.redirect_uri = request.query.get("redirect_uri")

        # TODO: add permission description per requested scope
        scope = request.query.get("scope")

        # TODO: used for preventing cross-site request forgery
        # [Section 10.12](https://tools.ietf.org/html/rfc6749#section-10.12)
        self.state = request.query.get("state")

        _LOGGER.debug(f"response_type : {response_type}")
        _LOGGER.debug(f"client_id     : {client_id}")
        _LOGGER.debug(f"client_secret : {client_secret}")
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
