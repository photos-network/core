"""Photo representation for persistency."""
from datetime import datetime

from sqlalchemy import Boolean, Column, DateTime, ForeignKey, Integer, String

from ...base import Base, generate_uuid


class Photo(Base):
    """Photo representation for photo files."""

    __tablename__ = "photos"

    uuid = Column(String, name="uuid", primary_key=True, default=generate_uuid)
    directory = Column(String)
    filename = Column(String)
    owner = Column(Integer, ForeignKey("users.uuid"), nullable=False)
    date_added = Column(DateTime, default=datetime.utcnow)  # added to core
    date_taken = Column(DateTime)  # photographed
    is_missing = Column(Boolean, default=False)  # file is missing on disk
    hash = Column(String)

    def __init__(self, directory, filename, owner, is_missing=False, hash=""):
        self.directory = directory
        self.filename = filename
        self.owner = owner
        self.is_missing = is_missing
        self.hash = hash
