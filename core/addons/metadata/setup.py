"""Userfacing setup process."""
import logging
from typing import Any, Dict, Optional

from core.addons import AddonSetupFlow

_LOGGER = logging.getLogger(__name__)


class MetadataSetup(AddonSetupFlow):
    """Userfacing setup."""

    async def async_step_user(
        self, user_input: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """Invoke when a user initiates a flow via the user interface."""
