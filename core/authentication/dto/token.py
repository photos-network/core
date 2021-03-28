"""Access token representation."""
from datetime import datetime, timedelta

from sqlalchemy import Column, DateTime, ForeignKey, Integer, String

from ...base import Base, generate_uuid
from ...const import CONF_TOKEN_LIFETIME


class Token(Base):
    __tablename__ = "tokens"

    id = Column(String, name="uuid", primary_key=True, default=generate_uuid)
    user_id = Column(Integer, ForeignKey("users.uuid"), nullable=False)
    client_id = Column(String)
    access_token = Column(String)
    token_expiration = Column(DateTime)
    refresh_token = Column(String)
    authorized_date = Column(DateTime, default=datetime.utcnow)
    last_used = Column(DateTime)

    def __init__(
        self, user_id: int, client_id: str, access_token: str, refresh_token: str
    ):
        self.user_id = user_id
        self.client_id = client_id
        self.access_token = access_token
        self.token_expiration = datetime.utcnow() + timedelta(
            seconds=CONF_TOKEN_LIFETIME
        )
        self.refresh_token = refresh_token

    def __repr__(self):
        return f"AccessToken(access={self.access_token}, refresh={self.refresh_token}, expiration={self.token_expiration})"
