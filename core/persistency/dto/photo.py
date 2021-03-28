"""Photo representation for persistency."""
from sqlalchemy import Boolean, Column, ForeignKey, Integer, String

from ...base import Base, generate_uuid


class Photo(Base):
    """Photo representation for photo files."""

    __tablename__ = "photos"

    uuid = Column(String, name="uuid", primary_key=True, default=generate_uuid)
    directory = Column(String)
    filename = Column(String)
    owner = Column(Integer, ForeignKey("users.uuid"), nullable=False)
    is_missing = Column(Boolean, default=False)
    hash = Column(String)

    def __init__(self, directory, filename, owner, is_missing=False, hash=""):
        self.directory = directory
        self.filename = filename
        self.owner = owner
        self.is_missing = is_missing
        self.hash = hash
