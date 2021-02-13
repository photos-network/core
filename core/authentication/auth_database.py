import logging
import random
import string
import uuid
from datetime import datetime, timedelta
from typing import Optional, Tuple

import jwt
from passlib.hash import sha256_crypt
from sqlalchemy.sql.expression import false

from .auth_code import AuthorizationCode
from .base import Base, Session, engine
from .token import Token
from .user import User

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class AuthDatabase:
    def __init__(self, database_file: str):
        Base.metadata.create_all(engine)
        self.session = Session()

        users = self.session.query(User).filter(User.login == "admin").all()
        if len(users) < 1:
            # insert admin user with generated password into database
            generated_password = "".join(
                random.SystemRandom().choice(string.ascii_letters + string.digits)
                for _ in range(10)
            )
            _LOGGER.warning(
                f"generated an admin user with password: '{generated_password}'"
            )

            # hashed = sha256_crypt.hash(generated_password)
            hashed = sha256_crypt.hash("admin")
            user = User("admin", hashed, True, False)
            self.session.add(user)
            self.session.commit()

    async def check_credentials(self, username: str, password: str) -> bool:
        """Check if the given username is found, not disabled and matches with the hashed password."""
        user = (
            self.session.query(User)
            .filter(User.login == username)
            .filter(User.disabled == false())
            .first()
        )

        if user is not None:
            return sha256_crypt.verify(password, user.passwd)
        else:
            return False

    def create_authorization_code(
        self,
        username: str,
        client_id: str,
    ) -> Optional[uuid.UUID]:
        """
        Create and persist an authorization_code with a max lifetime of 10 minutes to mitigate the risk of leaks.
        """
        authorization_code = uuid.uuid4()

        user = (
            self.session.query(User)
            .filter(User.login == username)
            .filter(User.disabled == false())
            .first()
        )

        self.session.add(AuthorizationCode(user.id, client_id, str(authorization_code)))
        self.session.commit()

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
            self.session.query(AuthorizationCode)
            .filter(AuthorizationCode.authorization_code == authorization_code)
            .filter(AuthorizationCode.expiration_time < str(datetime.utcnow))
            .filter(AuthorizationCode.used == false())
            .count()
        )

        if count > 0:
            self.session.query(AuthorizationCode).filter(
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
            self.session.query(AuthorizationCode)
            .filter(AuthorizationCode.authorization_code == authorization_code)
            .first()
        )

        key = "secret"
        access_token = jwt.encode(
            {"exp": datetime.utcnow() + timedelta(minutes=3)}, key, algorithm="HS256"
        )
        refresh_token = jwt.encode({}, key, algorithm="HS256")

        token = Token(
            user_id=auth_code.user_id,
            client_id=auth_code.client_id,
            access_token=access_token,
            refresh_token_id=refresh_token,
        )
        self.session.add(token)
        self.session.commit()

        return access_token, refresh_token

    async def renew_tokens(self, client_id: str, refresh_token: str) -> Tuple[str, str]:
        """Renew the access token."""
        key = "secret"
        new_token = jwt.encode(
            {"exp": datetime.utcnow() + timedelta(minutes=3)}, key, algorithm="HS256"
        )

        self.session.query(Token).filter(Token.client_id == client_id).filter(
            Token.token_expiration < str(datetime.utcnow)
        ).update(
            {
                Token.access_token: new_token,
                Token.token_expiration: datetime.utcnow() + timedelta(minutes=3),
            },
            synchronize_session=False,
        )

        return new_token, refresh_token

    async def revoke_token(self, access_token: str) -> bool:
        """Revoke access token for client_id."""
        count = (
            self.session.query(Token)
            .filter(Token.access_token == access_token)
            .update(
                {Token.token_expiration: datetime.utcnow()},
                synchronize_session=False,
            )
        )

        return count > 0

    async def validate_access_token(self, access_token: str) -> bool:
        count = (
            self.session.query(Token)
            .filter(Token.access_token == access_token)
            .filter(Token.token_expiration > str(datetime.utcnow()))
            .count()
        )

        return count > 0
