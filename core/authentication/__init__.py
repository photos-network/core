"""Initialization of authentication."""
import json
import logging
import secrets
import uuid
from typing import TYPE_CHECKING, List, Optional

if TYPE_CHECKING:
    from core.core import ApplicationCore

import aiohttp_jinja2
from aiohttp import hdrs, web

from ..const import CONF_TOKEN_LIFETIME, URL_API
from .auth_client import AuthenticationClient
from .auth_database import AuthDatabase

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Authentication:
    # list of registered auth clients
    auth_clients: List[AuthenticationClient] = []

    def __init__(
        self,
        core: "ApplicationCore",
        application: web.Application,
        auth_database: AuthDatabase,
    ):
        self.core = core
        self.app = application
        self.authorization = self.core.authorization
        self.auth_database = auth_database

        # Authorization Endpoint: obtain an authorization grant
        self.app.router.add_get(
            path=URL_API + "/oauth/authorize", handler=self.authorization_endpoint_get
        )
        self.app.router.add_post(
            path=URL_API + "/oauth/authorize", handler=self.authorization_endpoint_post
        )

        # Token Endpoint: obtain an access token by authorization grant or refresh token
        self.app.router.add_post(
            path=URL_API + "/oauth/token", handler=self.token_endpoint_handler
        )

        self.app.router.add_post(
            URL_API + "/revoke", self.revoke_token_handler, name="revoke"
        )
        self.app.router.add_get(
            URL_API + "/protected", self.protected_handler, name="protected"
        )

    def add_client(self, auth_client: AuthenticationClient):
        self.auth_clients.append(auth_client)

    async def revoke_token_handler(self, request: web.Request) -> web.StreamResponse:
        """
        Revoke the request token and all associated access tokens [RFC 7009]

        See Section 2.1: https://tools.ietf.org/html/rfc7009#section-2.1
        """
        _LOGGER.info("POST /revoke")

        await self.check_authorized(request)

        data = await request.post()
        token_to_revoke = data["token"]

        await self.auth_database.revoke_token(token_to_revoke)

        return web.Response(status=200)

    async def protected_handler(self, request: web.Request) -> web.StreamResponse:
        _LOGGER.warning("GET /protected")
        await self.check_permission(request, "library:read")

        response = web.Response(body=b"You are on protected page")
        return response

    @aiohttp_jinja2.template("authorize.jinja2")
    async def authorization_endpoint_get(
        self, request: web.Request
    ) -> web.StreamResponse:
        """
        Validate the request to ensure that all required parameters are present and valid.

        See Section 4.1.1: https://tools.ietf.org/html/rfc6749#section-4.1.1
        """
        try:
            _LOGGER.debug(f"GET /oauth/authorize  from: {request.host}")

            response_type = request.query.get("response_type")
            client_id = request.query.get("client_id")

            # validate required params
            if response_type is None or client_id is None:
                _LOGGER.warning("The response is missing a response_type or client_id.")
                data = """{
                    "error": "invalid_request",
                    "error_description": "The request is missing a required parameter"
                    }"""
                return web.json_response(json.loads(data))

            # check if client is known
            if not any(client.client_id == client_id for client in self.auth_clients):
                _LOGGER.warning("The client_id is unknown!")
                data = """{
                    "error":"unauthorized_client",
                    "error_description":"The client is not authorized to request an authorization code using this method."
                    }"""
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
                _LOGGER.error(f"redirect uri not found: {redirect_uri}")
                data = """{
                    "error":"unauthorized_client",
                    "error_description":"The redirect_uri is unknown"
                    }"""
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
            _LOGGER.debug(
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

            # persist state to preventing cross-site request forgery [Section 10.12](https://tools.ietf.org/html/rfc6749#section-10.12)
            # state = request.query.get("state")

            # TODO: add scopes & localized descriptions only for requested scopes
            return {
                "requesting_app": registered_auth_client.client_name,
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
                    # {
                    #     "scope": "library.read",
                    #     "localized": "Read only Grant the user to list all photos owned by the user.",
                    # },
                    # {
                    #     "scope": "library.append",
                    #     "localized": "Limited write access Grant the user to add new photos, create new albums.",
                    # },
                    # {
                    #     "scope": "library.edit",
                    #     "localized": "Grant the user to edit photos owned by the user.",
                    # },
                    {
                        "scope": "library.write",
                        "localized": "Grant the user to add and edit photos, albums, tags.",
                    },
                    # {
                    #     "scope": "library.share",
                    #     "localized": "Grant the user to create new shares (photos/videos/albums).",
                    # },
                    # {
                    #     "scope": "admin.users:read",
                    #     "localized": "Grant the user to list users on the system.",
                    # },
                    # {
                    #     "scope": "admin.users:invite",
                    #     "localized": "Grant the user to invite new users to the system.",
                    # },
                    # {
                    #     "scope": "admin.users:write",
                    #     "localized": "Grant the user to manage users on the system.",
                    # },
                ],
            }
        except Exception as e:
            # This error code is needed because a 500 Internal Server
            # Error HTTP status code cannot be returned to the client via an HTTP redirect.
            _LOGGER.error(f"an unexpected error happened: {e}")
            data = """{
                "error":"server_error",
                "error_description":"The authorization server encountered an unexpected condition that prevented it from fulfilling the request."
                }"""
            return web.json_response(json.loads(data))

    async def authorization_endpoint_post(
        self, request: web.Request
    ) -> web.StreamResponse:
        """
        Validate the resource owners credentials.

        """
        _LOGGER.debug("POST /oauth/authorize")
        data = await request.post()

        redirect_uri = request.query["redirect_uri"]

        if "client_id" not in request.query:
            _LOGGER.warning("invalid form")
            raise web.HTTPFound(f"{redirect_uri}?error=unauthorized_client")
        client_id = request.query["client_id"]
        _LOGGER.debug(f"client_id {client_id}")

        state = None
        if "state" in request.query:
            state = request.query["state"]
            _LOGGER.debug(f"state {state}")

        # check if client is known
        if not any(client.client_id == client_id for client in self.auth_clients):
            _LOGGER.warning(f"unknown client_id {client_id}")
            if state is not None:
                raise web.HTTPFound(
                    f"{redirect_uri}?error=unauthorized_client&state={state}"
                )
            else:
                raise web.HTTPFound(f"{redirect_uri}?error=unauthorized_client")

        # extract client from registered auth_clients with matching client_id
        registered_auth_client = next(
            filter(lambda client: client.client_id == client_id, self.auth_clients),
            None,
        )
        # validate if redirect_uri is in registered_auth_client
        if not any(uri == redirect_uri for uri in registered_auth_client.redirect_uris):
            _LOGGER.error(f"invalid redirect_uri {redirect_uri}")
            if state is not None:
                raise web.HTTPFound(
                    f"{redirect_uri}?error=unauthorized_client&state={state}"
                )
            else:
                raise web.HTTPFound(f"{redirect_uri}?error=unauthorized_client")

        email = data["email"].strip(" ").lower()
        password = data["password"]

        # validate credentials
        credentials_are_valid = await self.auth_database.check_credentials(
            email, password
        )

        if credentials_are_valid:
            # create an authorization code
            authorization_code = self.auth_database.create_authorization_code(
                email, client_id, request.remote
            )
            _LOGGER.debug(f"authorization_code: {authorization_code}")
            if authorization_code is None:
                _LOGGER.warning("could not create auth code for client!")
                error_reason = "access_denied"
                if state is not None:
                    raise web.HTTPFound(
                        f"{redirect_uri}?error={error_reason}&state={state}"
                    )
                else:
                    raise web.HTTPFound(f"{redirect_uri}?error={error_reason}")

            if state is not None:
                _LOGGER.debug(
                    f"HTTPFound: {redirect_uri}?code={authorization_code}&state={state}"
                )
                redirect_response = web.HTTPFound(
                    f"{redirect_uri}?code={authorization_code}&state={state}"
                )
            else:
                _LOGGER.debug(f"HTTPFound: {redirect_uri}?code={authorization_code}")
                redirect_response = web.HTTPFound(
                    f"{redirect_uri}?code={authorization_code}"
                )

            raise redirect_response
        else:
            error_reason = "access_denied"
            _LOGGER.warning(f"redirect with error {error_reason}")
            if state is not None:
                raise web.HTTPFound(
                    f"{redirect_uri}?error={error_reason}&state={state}"
                )
            else:
                raise web.HTTPFound(f"{redirect_uri}?error={error_reason}")

    async def token_endpoint_handler(self, request: web.Request) -> web.StreamResponse:
        """
        Access Token:   https://tools.ietf.org/html/rfc6749#section-4.1.3
        Refresh Token:  https://tools.ietf.org/html/rfc6749#section-6
        """
        _LOGGER.debug("POST /oauth/token")

        data = await request.post()

        # grant_type is REQUIRED
        if "grant_type" not in data:
            _LOGGER.warning("no grant_type specified!")
            data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))
        grant_type = data["grant_type"]

        # switch flow based on grant_type
        if grant_type == "authorization_code":
            return await self._handle_authorization_code_request(data)
        elif grant_type == "refresh_token":
            return await self._handle_refresh_token_request(request, data)
        else:
            _LOGGER.warning(f"invalid grant_type! {grant_type}")
            data = '{"error":"invalid_request"}'
            return web.json_response(json.loads(data))

    async def _handle_authorization_code_request(self, data) -> web.StreamResponse:
        """
        See Section 4.1.3: https://tools.ietf.org/html/rfc6749#section-4.1.3
        """
        # grant_type already checked

        # code is REQUIRED
        if "code" not in data:
            _LOGGER.warning("code param not provided!")
            data = {"error": "invalid_request"}
            return web.json_response(status=400, data=data)
        code = data["code"]

        # redirect_uri is REQUIRED
        if "redirect_uri" not in data:
            _LOGGER.warning("redirect_uri param not provided!")
            data = {"error": "invalid_request"}
            return web.json_response(status=400, data=data)
        redirect_uri = data["redirect_uri"]

        # TODO: compare redirect_uri with previous call
        _LOGGER.debug(f"TODO: compare redirect_uri {redirect_uri}")

        # client_id is REQUIRED
        if "client_id" not in data:
            data = {"error": "invalid_request"}
            return web.json_response(status=400, data=data)
        client_id = data["client_id"]

        client_code_valid = await self.auth_database.validate_authorization_code(
            code, client_id
        )
        if not client_code_valid:
            _LOGGER.error("authorization_code invalid!")
            payload = {"error": "invalid_grant"}
            return web.json_response(status=400, data=payload)

        access_token, refresh_token = await self.auth_database.create_tokens(
            code, client_id
        )

        payload = {
            "access_token": access_token,
            "token_type": "Bearer",
            "expires_in": CONF_TOKEN_LIFETIME,
            "refresh_token": refresh_token,
        }
        return web.json_response(status=200, data=payload)

    async def _handle_refresh_token_request(
        self, request: web.Request, data
    ) -> web.StreamResponse:
        """
        See Section 6: https://tools.ietf.org/html/rfc6749#section-6
        """
        # code is REQUIRED
        if "refresh_token" not in data:
            _LOGGER.warning("refresh token not provided!")
            data = {"error": "invalid_request"}
            return web.json_response(data)

        refresh_token = data["refresh_token"]

        # check if client_id and client_secret are provided as request parameters or HTTP Basic auth header
        if "client_id" in data and "client_secret" in data:
            # handle request parameters
            client_id = data["client_id"]
            client_secret = data["client_secret"]
        elif hdrs.AUTHORIZATION in request.headers:
            # handle basic headers
            auth_type, auth_val = request.headers.get(hdrs.AUTHORIZATION).split(" ", 1)
            if auth_type != "Basic":
                return False

            # TODO: split auth_val in client_id and client_secret
            _LOGGER.error(f"split token into client_id and client_secret: {auth_val}")
            client_id = ""
            client_secret = ""

        registered_auth_client = next(
            filter(lambda client: client.client_id == client_id, self.auth_clients),
            None,
        )

        _LOGGER.debug(f"client_id: {client_id}, {registered_auth_client}")

        if not registered_auth_client.client_secret == client_secret:
            _LOGGER.error("client_id does not match with client_secret")
            data = {"error": "invalid_client"}
            return web.json_response(data)

        access_token, refresh_token = await self.auth_database.renew_tokens(
            client_id, refresh_token
        )

        if access_token is None:
            raise web.HTTPForbidden()

        payload = {
            "access_token": access_token,
            "token_type": "Bearer",
            "expires_in": CONF_TOKEN_LIFETIME,
            "refresh_token": refresh_token,
        }
        return web.json_response(payload)

    def create_client(self):
        """Generate a client_id and client_secret to add new clients."""
        client_id = uuid.uuid4()
        client_secret = secrets.token_urlsafe(16)

        _LOGGER.info(f"generated client_id: {client_id}")
        _LOGGER.info(f"generated client_secret: {client_secret}")

    async def check_authorized(self, request: web.Request) -> Optional[str]:
        """Check if authorization header and returns user ID if valid"""

        if hdrs.AUTHORIZATION in request.headers:
            try:
                auth_type, auth_val = request.headers.get(hdrs.AUTHORIZATION).split(
                    " ", 1
                )
                if not await self.auth_database.validate_access_token(auth_val):
                    raise web.HTTPForbidden()

                return await self.auth_database.user_id_for_token(auth_val)

            except ValueError:
                # If no space in authorization header
                _LOGGER.debug("invalid authorization header!")

                raise web.HTTPForbidden()
        else:
            _LOGGER.debug("missing authorization header!")

            raise web.HTTPForbidden()

    async def check_permission(self, request: web.Request, scope: str) -> None:
        """Check if given authorization header is valid and user has granted access to given scope."""
        # check if user is authorized
        await self.check_authorized(request)

        # check if required scope is granted
        await self.core.authorization.check_scope(scope)
