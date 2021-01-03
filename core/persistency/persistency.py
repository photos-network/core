"""Persistency manager"""
import logging
import os
from typing import TYPE_CHECKING, Set

# Typing imports that create a circular dependency
from core.addon import AddonType
from core.configs import Config

if TYPE_CHECKING:
    from core.core import ApplicationCore


_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


class PersistencyManager:
    """Class to manage persistency tasks and trigger storage addons."""

    def __init__(self, config: Config, core: "ApplicationCore") -> None:
        """Initialize Persistency Manager."""
        self._config = config
        self._core = core
        self._addons: Set[str] = config.addons
        self._default_path = self._config.data_dir

    def file_exist(self, file_name: str, path: str = None) -> bool:
        if path is None:
            path = self._default_path

        _LOGGER.info(f"check if file {file_name} exists in path {path}")

        return os.path.exists(os.path.join(file_name, path))

    def create_file(self, file_name: str, path: str = None):
        if path is None:
            default_path = self._default_path
        else:
            default_path = path

        _LOGGER.info(f"create file {file_name} at {default_path}")

        file_object = open(os.path.join(file_name, default_path), "x")
        file_object.close()

    def create_key(self, key):
        for addon in self._core.loaded_addons.values():
            if addon.type == AddonType.STORAGE:
                # TODO: write key to persistency
                _LOGGER.warning(f"  Addon: {addon} {key}")
