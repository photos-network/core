"""authorization handles permissions / rights"""
import logging
from datetime import datetime, timedelta
from typing import TYPE_CHECKING

from aiohttp import web
from passlib.hash import sha256_crypt

from ..authentication.dto.token import Token
from ..authentication.dto.user import User
from ..base import Base, Session, engine
from ..const import CONF_DEADLINE

if TYPE_CHECKING:
    from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Authorization:
    def __init__(self, core: "ApplicationCore", application: web.Application):
        self.core = core
        self.app = application

        self.app.router.add_get(path="/v1/users", handler=self.get_users_handler)
        self.app.router.add_get(path="/v1/user/", handler=self.me_user_handler)
        self.app.router.add_get(path="/v1/user/{userId}", handler=self.get_user_handler)
        self.app.router.add_post(path="/v1/user/", handler=self.create_user_handler)
        self.app.router.add_patch(
            path="/v1/user/{userId}", handler=self.update_user_handler
        )
        self.app.router.add_delete(
            path="/v1/user/{userId}", handler=self.delete_user_handler
        )

        Base.metadata.create_all(engine)

    async def me_user_handler(self, request: web.Request) -> web.StreamResponse:
        """Return detailed information of the current loggedin user."""
        _LOGGER.debug("GET /user/ myself")

        await self.core.authentication.check_permission(request, "openid")
        await self.core.authentication.check_permission(request, "profile")
        await self.core.authentication.check_permission(request, "email")

        loggedInUser = await self.core.authentication.check_authorized(request)
        _LOGGER.debug(f"loggedInUser {loggedInUser}")

        user = Session.query(User).filter(User.id == loggedInUser).first()
        _LOGGER.debug(f"user {user}")

        lastSeen = "Never"
        if user.last_login is not None:
            lastSeen = user.last_login.isoformat()

        data = {
            "id": user.id,
            "email": user.email,
            "lastname": user.lastname,
            "firstname": user.firstname,
            "lastSeen": lastSeen,
        }

        return web.json_response(status=200, data=data)

    async def get_users_handler(self, request: web.Request) -> web.StreamResponse:
        """Return a simplified list of registered Users."""
        _LOGGER.debug("GET /users")
        await self.core.authentication.check_permission(request, "admin.users:read")

        users = Session.query(User).all()

        if len(users) < 1:
            raise web.HTTPInternalServerError()

        result = []
        for user in users:
            lastSeen = "Never"
            if user.last_login is not None:
                lastSeen = user.last_login.isoformat()

            result.append(
                {
                    "id": user.id,
                    "lastName": user.lastname,
                    "firstName": user.firstname,
                    "lastSeen": lastSeen,
                }
            )

        return web.json_response(status=200, data=result)

    async def get_user_handler(self, request: web.Request) -> web.StreamResponse:
        """Get a user by user ID"""
        userId = request.match_info["userId"]
        _LOGGER.debug(f"GET /user/{userId}")

        await self.core.authentication.check_permission(request, "openid")
        await self.core.authentication.check_permission(request, "profile")
        await self.core.authentication.check_permission(request, "email")

        if userId is None:
            raise web.HTTPBadRequest()

        user = Session.query(User).filter(User.id == userId).first()
        if not user:
            raise web.HTTPNotFound

        lastSeen = "Never"
        if user.last_login is not None:
            lastSeen = user.last_login.isoformat()

        data = {
            "id": user.id,
            "lastname": user.lastname,
            "firstname": user.firstname,
            "lastSeen": lastSeen,
        }

        return web.json_response(status=200, data=data)

    async def update_user_handler(self, request: web.Request) -> web.StreamResponse:
        """Update properties of the loggedin user."""
        userId = request.match_info["userId"]
        _LOGGER.debug(f"PATCH /user/{userId}")

        await self.core.authentication.check_permission(request, "openid")
        await self.core.authentication.check_permission(request, "profile")
        await self.core.authentication.check_permission(request, "email")

        loggedInUser = await self.core.authentication.check_authorized(request)
        if loggedInUser != userId:
            _LOGGER.error(
                f"attempt to change a different user! Authenticated user: {loggedInUser}, user to change: {userId}"
            )
            raise web.HTTPForbidden
        data = await request.json()

        if "email" in data:
            Session.query(User).filter(User.id == loggedInUser).update(
                {User.email: data["email"]},
                synchronize_session=False,
            )

        if "firstname" in data:
            Session.query(User).filter(User.id == loggedInUser).update(
                {User.firstname: data["firstname"]},
                synchronize_session=False,
            )

        if "lastname" in data:
            Session.query(User).filter(User.id == loggedInUser).update(
                {User.lastname: data["lastname"]},
                synchronize_session=False,
            )

        if "password" in data:
            hashed = sha256_crypt.hash(data["password"])
            Session.query(User).filter(User.id == loggedInUser).update(
                {User.password: hashed},
                synchronize_session=False,
            )

        Session.commit()
        return web.json_response(status=204)

    async def delete_user_handler(self, request: web.Request) -> web.StreamResponse:
        userId = request.match_info["userId"]
        _LOGGER.debug(f"DELETE /users/{userId}")
        await self.core.authentication.check_permission(request, "profile")

        loggedInUser = await self.core.authentication.check_authorized(request)
        if loggedInUser != userId:
            _LOGGER.error(
                f"attempt to change a different user! Authenticated user: {loggedInUser}, user to change: {userId}"
            )
            raise web.HTTPForbidden

        Session.query(User).filter(User.id == loggedInUser).update(
            {User.deleted_date: datetime.utcnow(), User.disabled: True},
            synchronize_session=False,
        )

        Session.commit()

        deadline = datetime.utcnow() + timedelta(seconds=CONF_DEADLINE)
        data = {"deadline": deadline.isoformat()}

        # invalidate all user tokens
        Session.query(Token).filter(Token.user_id == loggedInUser).delete()
        return web.json_response(status=202, data=data)

    async def create_user_handler(self, request: web.Request) -> web.StreamResponse:
        _LOGGER.warning("POST /user/")
        await self.core.authentication.check_permission(request, "admin.users:write")

        data = await request.json()
        if (
            "email" not in data
            or "lastname" not in data
            or "firstname" not in data
            or "password" not in data
        ):
            raise web.HTTPBadRequest

        email = data["email"]
        lastname = data["lastname"]
        firstname = data["firstname"]
        password = data["password"]

        count = Session.query(User).filter(User.email == email).count()

        if count > 0:
            data = {
                "error": "alreday_registered",
                "error_description": "This email is already registered!",
            }
            return web.json_response(status=400, data=data)

        hashed = sha256_crypt.hash(password)
        user = User(
            email=email, password=hashed, lastname=lastname, firstname=firstname
        )
        Session.add(user)
        Session.commit()

        new_user = Session.query(User).filter(User.email == email).first()

        if new_user:
            data = {
                "id": new_user.id,
                "email": new_user.email,
                "firstname": new_user.firstname,
                "lastname": new_user.lastname,
            }

            return web.json_response(status=201, data=data)

        _LOGGER.error(f"Could not find newly created user {email}")
        return web.json_response(status=500, data=data)

    async def check_scope(self, scope: str):
        _LOGGER.warning(f"check_scope({scope})")

        # TODO: raise if scope is missing
        # raise web.HTTPForbidden()
