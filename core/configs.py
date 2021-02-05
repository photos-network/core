"""Configurations for runtime and application instance."""
import dataclasses
import datetime
import logging
import os
from typing import TYPE_CHECKING, Dict, Optional, Set

import pytz

# Typing imports that create a circular dependency
if TYPE_CHECKING:
    from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


class Config:
    """Representation class of configurations."""

    def __init__(self, core: "ApplicationCore") -> None:
        """Initialize a new config object."""
        self.core = core

        self.internal_url: Optional[str] = "127.0.0.1"
        self.external_url: Optional[str] = "127.0.0.1"
        self.port: Optional[int] = 7777

        self.time_zone: datetime.tzinfo = pytz.utc.zone

        # List of loaded addons
        self.addons: Set[str] = set()

        # Directory that holds addons
        self.addon_dir: Optional[str] = "core/addons"

        # Directory that holds configuration data
        self.config_dir: Optional[str] = None

        # Directory that holds application data
        self.data_dir: Optional[str] = None

    def path(self, *path: str) -> str:
        """Generate path to the file within the configuration directory.

        Async friendly.
        """
        if self.config_dir is None:
            raise RuntimeError("config_dir is not set")
        return os.path.join(self.config_dir, *path)

    def as_dict(self) -> Dict:
        """Create a dictionary representation of the configuration.

        Async friendly.
        """
        time_zone = pytz.utc.UTC.zone
        if self.time_zone and getattr(self.time_zone, "zone"):
            time_zone = getattr(self.time_zone, "zone")

        return {
            "internal_url": self.internal_url,
            "external_url": self.external_url,
            "port": self.port,
            "time_zone": time_zone,
            "addons": self.addons,
            "config_dir": self.config_dir,
            "data_dir": self.data_dir,
        }


@dataclasses.dataclass
class RuntimeConfig:
    """Class to hold the information for running ApplicationCore."""

    # directory holding configurations
    config_dir: str

    # directory holding application data
    data_dir: str

    # disable addons if enabled
    safe_mode: bool = False

    # add debug log states
    debug: bool = False

    # add verbose log states
    verbose: bool = True
