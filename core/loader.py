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
        # Test integration cache
        integration = self._core.data.get(DATA_ADDONS, {}).get(comp_name)
        _LOGGER.error(f"Components.getattr integration {integration}")

        if isinstance(integration, Addon):
            component: Optional[ModuleType] = integration.get_component()
        else:
            # Fallback to importing old-school
            component = _load_file(self._core, comp_name, _lookup_path(self._core))

        if component is None:
            raise ImportError(f"Unable to load {comp_name}")

        wrapped = ModuleWrapper(self._core, component)
        setattr(self, comp_name, wrapped)
        return wrapped

    def _load_file(
            self,
            comp_or_platform: str,
            base_paths: List[str]
    ) -> Optional[ModuleType]:
        """Try to load specified file.

        Looks in config dir first, then built-in components.
        Only returns it if also found to be valid.
        Async friendly.
        """
        try:
            return self._core.data[DATA_ADDONS][comp_or_platform]  # type: ignore
        except KeyError:
            pass

        cache = self._core.data.get(DATA_ADDONS)
        if cache is None:
            cache = self._core.data[DATA_ADDONS] = {}

        for path in (f"{base}.{comp_or_platform}" for base in base_paths):
            try:
                module = importlib.import_module(path)

                # In Python 3 you can import files from directories that do not
                # contain the file __init__.py. A directory is a valid module if
                # it contains a file with the .py extension. In this case Python
                # will succeed in importing the directory as a module and call it
                # a namespace. We do not care about namespaces.
                # This prevents that when only
                # custom_components/switch/some_platform.py exists,
                # the import custom_components.switch would succeed.
                # __file__ was unset for namespaces before Python 3.7
                if getattr(module, "__file__", None) is None:
                    continue

                cache[comp_or_platform] = module

                if module.__name__.startswith(PACKAGE_CUSTOM_COMPONENTS):
                    _LOGGER.warning(CUSTOM_WARNING, comp_or_platform)

                return module

            except ImportError as err:
                # This error happens if for example custom_components/switch
                # exists and we try to load switch.demo.
                # Ignore errors for custom_components, custom_components.switch
                # and custom_components.switch.demo.
                white_listed_errors = []
                parts = []
                for part in path.split("."):
                    parts.append(part)
                    white_listed_errors.append(f"No module named '{'.'.join(parts)}'")

                if str(err) not in white_listed_errors:
                    _LOGGER.exception(f"Error loading {path}. Make sure all dependencies are installed")

        return None
