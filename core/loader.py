"""The methods for loading integrations."""
import asyncio
import functools as ft
import importlib
import json
import logging
import pathlib
from typing import Optional, List

import sys
from types import ModuleType

from core.addon import Addon
_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)

DATA_ADDONS = "addons"


class ModuleWrapper:
    """Class to wrap a Python module and auto fill in application core as argument."""

    def __init__(self,
                 core: "ApplicationCore",
                 module: ModuleType
                 ) -> None:
        """Initialize the module wrapper."""
        self._core = core
        self._module = module
        _LOGGER.error(f"ModuleWrapper init {module}")


class Components:
    """Helper to load components."""

    def __init__(
            self,
            core: "ApplicationCore"
    ) -> None:
        """Initialize the Components class."""
        self._core = core

    def __getattr__(self, comp_name: str) -> ModuleWrapper:
        """Fetch a component."""
        _LOGGER.debug(f"Components.getattr comp_name {comp_name}")

        # Test integration cache
        integration = self._core.data.get(DATA_ADDONS, {}).get(comp_name)
        _LOGGER.error(f"Components.getattr integration {integration}")

        if isinstance(integration, Addon):
            component: Optional[ModuleType] = integration.get_component()
            wrapped = ModuleWrapper(self._core, component)
            setattr(self, comp_name, wrapped)
            return wrapped
        else:
            _LOGGER.warning(f"wrong type: {type(integration)}")
            raise ImportError(f"Unable to load {comp_name}")
