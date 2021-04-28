"""User representation."""
import datetime

from sqlalchemy import Boolean, Column, DateTime, String

from ...base import Base, generate_uuid


class User(Base):
    """User representation for authentication."""

    __tablename__ = "users"

    id = Column(String, name="uuid", primary_key=True, default=generate_uuid)
    email = Column(String)
    password = Column(String)
    lastname = Column(String)
    firstname = Column(String)
    is_superuser = Column(Boolean)
    disabled = Column(Boolean)
    create_date = Column(DateTime, default=datetime.datetime.utcnow)
    deleted_date = Column(DateTime, nullable=True)
    last_login = Column(DateTime)
    last_ip = Column(String)

    def __init__(
        self, email, password, lastname, firstname, is_superuser=False, disabled=False
    ):
        self.email = email
        self.password = password
        self.lastname = lastname
        self.firstname = firstname
        self.is_superuser = is_superuser
        self.disabled = disabled

    def __repr__(self):
        return f"User(id={self.id!r}, email={self.email!r}, disabled={self.disabled!r})"
