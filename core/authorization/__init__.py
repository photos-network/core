"""authorization handles permissions / rights"""
import json
import logging
from typing import TYPE_CHECKING, Optional

from aiohttp import hdrs, web

from ..base import Base, Session, engine

if TYPE_CHECKING:
    from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Authorization:
    def __init__(self, core: "ApplicationCore", application: web.Application):
        self.core = core
        self.app = application

        self.app.router.add_put(path="/v1/users/", handler=self.create_user_handler)
        self.app.router.add_get(path="/v1/users", handler=self.get_users_handler)
        self.app.router.add_get(path="/v1/users/{userId}", handler=self.get_user_handler)
        self.app.router.add_patch(path="/v1/users/{userId}", handler=self.update_user_handler)
        self.app.router.add_delete(path="/v1/users/{userId}", handler=self.delete_user_handler)

        Base.metadata.create_all(engine)

    async def update_user_handler(self, request: web.Request) -> web.StreamResponse:
        userId = request.match_info["userId"]
        _LOGGER.debug(f"PATCH /users/{userId}")
        await self.core.authentication.check_permission(request, "admin.users:write")

        # TODO: update user properties

        return web.json_response(status=204)

    async def delete_user_handler(self, request: web.Request) -> web.StreamResponse:
        userId = request.match_info["userId"]
        _LOGGER.debug(f"DELETE /users/{userId}")
        await self.core.authentication.check_permission(request, "admin.users:write")

        # TODO: delete user

        return web.json_response(status=204)

    async def create_user_handler(self, request: web.Request) -> web.StreamResponse:
        _LOGGER.warning(f"PUT /users/")
        await self.core.authentication.check_permission(request, "admin.users:write")

        data = request.content

        # TODO: create user with given properties
        _LOGGER.warning(f"TODO: create/update data: {data}")
        login = data["login"]

        user = Session.query(User).filter(User.login == username).filter(User.disabled == false()).first()

        if user is not None:
            hashed = sha256_crypt.hash(generated_password)
            user = User("max", hashed, True, False)
            Session.add(user)
            Session.commit()

            data = {
                "properties.id": "2",
                "properties.email": "max.mustermann@photos.network",
                "properties.firstName": "Max",
                "properties.lastName": "Mustermann",
            }

            return web.json_response(status=200, data=data)
        else:
            data = {"error": "not_implemented", "error_description": "This request is missing an implementation"}

            return web.json_response(status=400, data=data)

    async def get_users_handler(self, request: web.Request) -> web.StreamResponse:
        _LOGGER.debug(f"GET /users")
        await self.core.authentication.check_permission(request, "admin.users:read")

        # TODO: get users
        data = {
            "value": [
                {
                    "id": "1",
                    "displayName": "Max Mustermann",
                    "email": "max.mustermann@photos.network",
                    "firstName": "Max",
                    "lastName": "Mustermann",
                }
            ]
        }

        return web.json_response(status=200, data=data)

    async def get_user_handler(self, request: web.Request) -> web.StreamResponse:
        userId = request.match_info["userId"]
        _LOGGER.debug(f"GET /users/{userId}")
        await self.core.authentication.check_permission(request, "admin.users:read")

        # TODO: get user properties
        data = {
            "properties.id": "1",
            "properties.email": "max.mustermann@photos.network",
            "properties.firstName": "Max",
            "properties.lastName": "Mustermann",
        }

        return web.json_response(status=200, data=data)

    async def check_scope(self, scope: str):
        _LOGGER.warning(f"check_scope({scope})")

        # TODO: raise if scope is missing
        # raise web.HTTPForbidden()
