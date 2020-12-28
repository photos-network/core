"""User representation."""
import datetime
from typing import Optional


class User:
    def __init__(
            self,
            user_id: str,
            username: str,
            name: Optional[str],
            email: Optional[str],
            password_hash: str,
            is_admin: bool = False,
            is_active: bool = True,
            last_login: datetime = datetime.datetime,
            date_joined: datetime = datetime.datetime
    ):
        self.username = username
        self.name = name
        self.email = email
        self.password_hash = password_hash
        self.is_admin = is_admin
        self.is_active = is_active
