"""Directory representation for persistency."""
import uuid
from datetime import datetime

from sqlalchemy import Column, DateTime, ForeignKey, Integer, String

from ...base import Base


def generate_uuid():
    return str(uuid.uuid4())


class Directory(Base):
    """Directory representation for user directories containing files."""

    __tablename__ = "directories"

    id = Column(String, name="uuid", primary_key=True, default=generate_uuid)
    directory = Column(String)
    owner = Column(Integer, ForeignKey("users.uuid"), nullable=False)
    added = Column(DateTime, default=datetime.utcnow)
    last_scanned = Column(DateTime)

    def __init__(self, directory, owner):
        self.directory = directory
        self.owner = owner

    def __repr__(self):
        return f"Directory({self.directory}, owner={self.owner})"
