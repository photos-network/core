"""Authorization code representation."""
from datetime import datetime, timedelta

from sqlalchemy import Boolean, Column, Integer, String

from .base import Base


class AuthorizationCode(Base):
    __tablename__ = "authorizationcodes"

    id = Column(Integer, primary_key=True)
    user_id = Column(Integer)
    client_id = Column(String)
    authorization_code = Column(String)
    expiration_time = Column(String)
    used = Column(Boolean, default=False)

    def __init__(self, user_id, client_id, authorization_code):
        self.user_id = user_id
        self.client_id = client_id
        self.authorization_code = authorization_code
        self.expiration_time = datetime.utcnow() + timedelta(minutes=10)

    def __repr__(self):
        return f"AuthorizationCode(client_id={self.client_id!r}, authorization_code={self.authorization_code!r}, expiration_time={self.expiration_time!r})"
