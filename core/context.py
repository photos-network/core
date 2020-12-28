"""Constants use by the Application instance"""
import attr

from typing import Optional
from core.utils.uuid import random_uuid_hex


@attr.s(slots=True, frozen=True)
class Context:
    """The context that triggered something."""

    user_id: str = attr.ib(default=None)
    parent_id: Optional[str] = attr.ib(default=None)
    id: str = attr.ib(factory=random_uuid_hex)

    def as_dict(self) -> dict:
        """Return a dictionary representation of the context."""
        return {"id": self.id, "parent_id": self.parent_id, "user_id": self.user_id}
