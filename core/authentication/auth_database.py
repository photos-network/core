import logging
import random
import string
import uuid
from datetime import datetime, timedelta
from typing import Optional, Tuple

import jwt
from passlib.hash import sha256_crypt
from sqlalchemy import or_
from sqlalchemy.sql.expression import false

from ..base import Base, Session, engine
from ..const import CONF_TOKEN_LIFETIME
from .dto.auth_code import AuthorizationCode
from .dto.token import Token
from .dto.user import User

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class AuthDatabase:
    def __init__(self, data_dir: str):
        Base.metadata.create_all(engine)

        users = Session.query(User).all()
        if len(users) < 1:
            # insert admin user with generated password into database
            generated_password = "".join(
                random.SystemRandom().choice(string.ascii_letters + string.digits)
                for _ in range(10)
            )
            _LOGGER.warning(
                f"generated an admin user with username: 'admin@photos.network' password: '{generated_password}'."
            )

            hashed = sha256_crypt.hash(generated_password)
            user = User(
                "admin@photos.network",
                hashed,
                "Administrator",
                "Generated User",
                True,
                False,
            )
            Session.add(user)
            Session.commit()

    async def check_credentials(self, email: str, password: str) -> bool:
        """Check if the given email is found, not disabled and matches with the hashed password."""
        user = (
            Session.query(User)
            .filter(User.email == email)
            .filter(User.disabled == false())
            .first()
        )

        if user is not None:
            return sha256_crypt.verify(password, user.password)
        else:
            return False

    def create_authorization_code(
        self,
        email: str,
        client_id: str,
        origin: str,
    ) -> Optional[uuid.UUID]:
        """
        Create and persist an authorization_code with a max lifetime of 10 minutes to mitigate the risk of leaks.
        """
        authorization_code = uuid.uuid4()

        user = (
            Session.query(User)
            .filter(User.email == email)
            .filter(User.disabled == false())
            .first()
        )

        Session.query(User).filter(User.email == email).update(
            {User.last_login: datetime.utcnow(), User.last_ip: origin},
            synchronize_session=False,
        )

        Session.add(AuthorizationCode(user.id, client_id, str(authorization_code)))
        Session.commit()

        return authorization_code

    async def validate_authorization_code(
        self,
        authorization_code: uuid.UUID,
        client_id: str,
    ) -> bool:
        """
        Validates the given authorization_code
        """
        count = (
            Session.query(AuthorizationCode)
            .filter(AuthorizationCode.authorization_code == authorization_code)
            .filter(AuthorizationCode.expiration_time < str(datetime.utcnow))
            .filter(AuthorizationCode.used == false())
            .count()
        )

        if count > 0:
            Session.query(AuthorizationCode).filter(
                AuthorizationCode.authorization_code == authorization_code
            ).filter(AuthorizationCode.expiration_time < str(datetime.utcnow)).update(
                {AuthorizationCode.used: True}, synchronize_session=False
            )

        # TODO: if AuthorizationCode already used. revoke previously issued tokens based on that authorization code
        # https://tools.ietf.org/html/rfc6749#section-4.1.2

        return count > 0

    async def create_tokens(
        self,
        authorization_code: uuid.UUID,
        client_id: str,
    ) -> Tuple[str, str]:
        """
        Create and persist access and refresh tokens.
        """
        auth_code = (
            Session.query(AuthorizationCode)
            .filter(AuthorizationCode.authorization_code == authorization_code)
            .first()
        )

        key = "secret"
        access_token = jwt.encode(
            {"exp": datetime.utcnow() + timedelta(seconds=CONF_TOKEN_LIFETIME)},
            key,
            algorithm="HS256",
        )
        refresh_token = jwt.encode({}, key, algorithm="HS256")

        token = Token(
            user_id=auth_code.user_id,
            client_id=auth_code.client_id,
            access_token=access_token,
            refresh_token=refresh_token,
        )
        Session.add(token)
        Session.commit()

        return access_token, refresh_token

    async def renew_tokens(
        self, client_id: str, refresh_token: str
    ) -> Tuple[Optional[str], Optional[str]]:
        """Renew the access token."""

        # validate refresh token
        count = (
            Session.query(Token)
            .filter(Token.client_id == client_id)
            .filter(Token.refresh_token == refresh_token)
            .count()
        )

        if count < 1:
            return None, None

        key = "secret"
        new_access_token = jwt.encode(
            {"exp": datetime.utcnow() + timedelta(seconds=CONF_TOKEN_LIFETIME)},
            key,
            algorithm="HS256",
        )
        new_refresh_token = jwt.encode(
            {"some": str(datetime.utcnow())},
            key,
            algorithm="HS256",
        )

        # update tokens in database
        Session.query(Token).filter(Token.client_id == client_id).filter(
            Token.token_expiration < str(datetime.utcnow)
        ).update(
            {
                Token.access_token: new_access_token,
                Token.token_expiration: datetime.utcnow()
                + timedelta(seconds=CONF_TOKEN_LIFETIME),
                Token.refresh_token: new_refresh_token,
            },
            synchronize_session=False,
        )
        Session.commit()

        return new_access_token, new_refresh_token

    async def revoke_token(self, token: str) -> bool:
        """Revoke given token and associated tokens."""

        # TODO: validate if the token was issued by the requesting
        # TODO: if the provided token is a refresh_token, invalidate all access_tokens based with this refresh_token
        count = (
            Session.query(Token)
            .filter(or_(Token.access_token == token, Token.refresh_token == token))
            .delete()
        )
        Session.commit()

        return count > 0

    async def validate_access_token(self, access_token: str) -> bool:
        count = (
            Session.query(Token)
            .filter(Token.access_token == access_token)
            .filter(Token.token_expiration > str(datetime.utcnow()))
            .count()
        )

        if count > 0:
            Session.query(Token).filter(Token.access_token == access_token).update(
                {Token.last_used: datetime.utcnow()},
                synchronize_session=False,
            )
            Session.commit()

        return count > 0

    async def user_id_for_token(self, access_token: str) -> str:
        row = (
            Session.query(Token)
            .filter(Token.access_token == access_token)
            .filter(Token.token_expiration > str(datetime.utcnow()))
            .first()
        )

        return row.user_id
