"""User representation."""
import datetime

from sqlalchemy import Boolean, Column, DateTime, Integer, String

from .base import Base


class User(Base):
    """User representation for authentication."""

    __tablename__ = "users"

    id = Column(Integer, primary_key=True)
    login = Column(String)
    passwd = Column(String)
    is_superuser = Column(Boolean)
    disabled = Column(Boolean)
    create_date = Column(DateTime, default=datetime.datetime.utcnow)
    last_login = Column(DateTime)
    last_ip = Column(String)

    def __init__(self, login, passwd, is_superuser=False, disabled=False):
        self.login = login
        self.passwd = passwd
        self.is_superuser = is_superuser
        self.disabled = disabled

    def __repr__(self):
        return f"User(id={self.id!r}, login={self.login!r}, disabled={self.disabled!r})"
