"""Loading addons from regarding directory."""
import asyncio
import importlib
import json
import logging
import os
import pathlib
from enum import Enum
from types import ModuleType

import time
from subprocess import Popen, PIPE
from typing import Optional, cast, Dict, Any, List, TYPE_CHECKING

import sys

# Typing imports that create a circular dependency
if TYPE_CHECKING:
    from core.core import ApplicationCore

SLOW_SETUP_MAX_WAIT = 300

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


class AddonType(Enum):
    """Describe"""
    IMAGE = "image"
    STORAGE = "storage"


class Addon:
    """Base representation of addons."""

    @classmethod
    def resolve_from_root(
            cls,
            core: "ApplicationCore",
            addon_name: str,
            addon_config: dict = None
    ) -> "Optional[Addon]":
        """Resolve an addon from a root module."""
        addon_path = pathlib.Path(core.config.addon_dir) / addon_name / "addon.json"

        if not addon_path.is_file():
            return None

        try:
            manifest = json.loads(addon_path.read_text())
        except ValueError as err:
            _LOGGER.error(f"Error parsing manifest.json file at {addon_path}: {err}")
            return None

        return cls(
            core,
            f"{addon_path}",
            addon_path.parent,
            manifest,
            addon_config
        )

    def __init__(
            self,
            core: "ApplicationCore",
            pkg_path: str,
            file_path: pathlib.Path,
            manifest: Dict[str, Any],
            addon_config: Dict[str, Any] = None
    ):
        """Initialize an integration."""
        self.core = core
        self.pkg_path = pkg_path
        self.file_path = file_path
        self.manifest = manifest
        self.addon_config = addon_config

        _LOGGER.info(f"Initialize '{self.domain}' ({pkg_path})")

    @property
    def name(self) -> str:
        """Return name."""
        return cast(str, self.manifest["name"])

    @property
    def disabled(self) -> Optional[str]:
        """Return reason integration is disabled."""
        return cast(Optional[str], self.manifest.get("disabled"))

    @property
    def domain(self) -> str:
        """Return domain."""
        return cast(str, self.manifest["domain"])

    @property
    def dependencies(self) -> List[str]:
        """Return dependencies."""
        return cast(List[str], self.manifest.get("dependencies", []))

    @property
    def requirements(self) -> List[str]:
        """Return requirements."""
        return cast(List[str], self.manifest.get("requirements", []))

    @property
    def type(self) -> AddonType:
        return cast(AddonType, AddonType(self.manifest.get("type", AddonType.IMAGE)))

    @property
    def config_flow(self) -> bool:
        """Return config_flow."""
        return cast(bool, self.manifest.get("config_flow", False))

    @property
    def documentation(self) -> Optional[str]:
        """Return documentation."""
        return cast(str, self.manifest.get("documentation"))

    def __repr__(self) -> str:
        """Text representation of class."""
        return f"<Addon {self.domain}: {self.pkg_path}>"

    def install_requirements(self) -> bool:
        """install requirements for addon. return True if all requrements fulfilled"""
        all_requirements_resolved = True
        for requirement in self.requirements:
            _LOGGER.info(f"Install requirement {requirement} for {self.domain}")
            try:
                installed = self.install_package(package=requirement)
                if not installed:
                    all_requirements_resolved = False
            except:
                _LOGGER.error(f"could not fulfill requirement: {requirement}")
                all_requirements_resolved = False

        return all_requirements_resolved

    def install_package(
            self,
            package: str,
            upgrade: bool = True,
            target: Optional[str] = None,
            constraints: Optional[str] = None,
            find_links: Optional[str] = None,
            no_cache_dir: Optional[bool] = False,
    ) -> bool:
        """Install a package on PyPi. Accepts pip compatible package strings.

        Return boolean if install successful.
        """
        # Not using 'import pip; pip.main([])' because it breaks the logger
        _LOGGER.info("Attempting install of %s", package)
        env = os.environ.copy()
        args = [sys.executable, "-m", "pip", "install", "--quiet", package]
        if no_cache_dir:
            args.append("--no-cache-dir")
        if upgrade:
            args.append("--upgrade")
        if constraints is not None:
            args += ["--constraint", constraints]
        if find_links is not None:
            args += ["--find-links", find_links, "--prefer-binary"]
        process = Popen(args, stdin=PIPE, stdout=PIPE, stderr=PIPE, env=env)
        _, stderr = process.communicate()
        if process.returncode != 0:
            _LOGGER.error(
                "Unable to install package %s: %s",
                package,
                stderr.decode("utf-8").lstrip().strip(),
            )
            return False

        return True

    async def async_setup_addon(
            self,
    ) -> bool:
        """run custom setup for addon."""
        processed_config = self.addon_config

        start = time.perf_counter()
        _LOGGER.info(f"Start setup for '{self.domain}'")

        # Some integrations fail on import because they call functions incorrectly.
        # So we do it before validating config to catch these errors.
        try:
            component = importlib.import_module(f"core.addons.{self.domain}")
        except ImportError as err:
            _LOGGER.error(f"Unable to import addon '{self.domain}': {err}")
            return False
        except Exception:  # pylint: disable=broad-except
            _LOGGER.exception(f"Setup failed for {self.domain}: unknown error")
            return False

        try:
            if hasattr(component, "async_setup"):
                task = component.async_setup(self.core, processed_config)  # type: ignore
            elif hasattr(component, "setup"):
                # This should not be replaced with core.async_add_executor_job because
                # we don't want to track this task in case it blocks startup.
                task = self.core.loop.run_in_executor(
                    None, component.setup, self.core, processed_config  # type: ignore
                )
            else:
                _LOGGER.error("!! ==> No setup function defined.")
                return False

            async with self.core.timeout.async_timeout(SLOW_SETUP_MAX_WAIT, self.domain):
                result = await task
        except asyncio.TimeoutError:
            _LOGGER.error(
                "Setup of %s is taking longer than %s seconds."
                " Startup will proceed without waiting any longer",
                self.domain,
                SLOW_SETUP_MAX_WAIT,
            )
            return False
        except Exception:  # pylint: disable=broad-except
            _LOGGER.exception("Error during setup of component %s", self.domain)
            return False
        finally:
            end = time.perf_counter()
        _LOGGER.info(f"Setup of '{self.domain}' took {end - start:#.2f} seconds")

        if result is False:
            _LOGGER.error("Integration failed to initialize.")
            return False
        if result is not True:
            _LOGGER.error(
                f"Integration {self.domain!r} did not return boolean if setup was "
                "successful. Disabling component."
            )
            return False

        # Flush out async_setup calling create_task. Fragile but covered by test.
        await asyncio.sleep(0)

        self.core.config.addons.add(self.domain)

        return True

    async def async_process_images_in_addons(self, images) -> bool:
        """Trigger image addons with images to process"""

        if self.type == AddonType.IMAGE:
            start = time.perf_counter()
            _LOGGER.info(f"Start processing images with addon '{self.domain}'")

            try:
                component = importlib.import_module(f"core.addons.{self.domain}")
            except ImportError as err:
                _LOGGER.error(f"Processing images with addon '{self.domain}' failed: {err}")
                return False
            except Exception:  # pylint: disable=broad-except
                _LOGGER.exception(f"Processing images with addon '{self.domain}' failed: unknown error")
                return False

            try:
                if hasattr(component, "async_process_images"):
                    task = component.async_process_images(self.core, self.core.config, images)  # type: ignore

                    async with self.core.timeout.async_timeout(30, self.domain):
                        result = await task
                        if result is None or not result:
                            _LOGGER.warning("could not finish successful")
            finally:
                end = time.perf_counter()
            _LOGGER.info(f"Processing {len(images)} images with addon '{self.domain}' took {end - start:#.2f} seconds.")

        elif self.type == AddonType.STORAGE:
            _LOGGER.debug(f"Skipping non image addon '{self.domain}'")

            return False

        return False

    def get_component(self) -> ModuleType:
        """Return the component."""
        cache = self.core.data.setdefault("addons", {})
        if self.domain not in cache:
            cache[self.domain] = importlib.import_module(self.pkg_path)
        return cache[self.domain]  # type: ignore


class AddonSetupFlow:
    """Base class for the setup flow. See PEP 487 for details"""
    # _core: Optional[ApplicationCore] = None
    _registry = []

    @classmethod
    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls._registry.append(cls)

    def get_registry(cls):
        return cls._registry

    async def async_step_user(self, user_input: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Handle a flow initiated by the user."""
        return self.async_abort(reason="not_implemented")

    def async_abort(self, *, reason: str, description_placeholders: Optional[Dict] = None) -> Dict[str, Any]:
        """Abort the config flow."""

        return dict()
