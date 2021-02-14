"""Entry point of Photos.network core."""

import asyncio
import logging
import os
import sys
from typing import Optional

from core.configs import RuntimeConfig
from core.const import REQUIRED_PYTHON_VER
from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


def validate_python() -> None:
    """Validate that the right Python version is running."""
    if sys.version_info[:3] < REQUIRED_PYTHON_VER:
        print(
            "Photos.network requires at least Python "
            f"{REQUIRED_PYTHON_VER[0]}.{REQUIRED_PYTHON_VER[1]}.{REQUIRED_PYTHON_VER[2]}"
        )
        sys.exit(1)


async def async_setup(
    runtime_config: RuntimeConfig,
) -> Optional[ApplicationCore]:
    """Set up Photos."""
    application = ApplicationCore()
    _LOGGER.debug("main::async_setup()")
    application.config.config_dir = runtime_config.config_dir
    application.config.data_dir = runtime_config.data_dir
    application.async_enable_logging(verbose=True)

    # create date directory
    get_or_create_directory(directory=application.config.data_dir, is_relative=False)

    _LOGGER.info(
        f"Config directory: {runtime_config.config_dir}",
    )

    return application


async def setup_and_run(runtime_config: RuntimeConfig) -> int:
    """Set up and run the system."""
    core = await async_setup(runtime_config)

    if core is None:
        return 1

    return await core.async_run()


def get_or_create_directory(directory: str, is_relative: bool = True) -> str:
    """Return absolute path of given directory. Create if it does not exist."""
    if is_relative:
        directory_path = os.path.abspath(os.path.join(os.getcwd(), directory))
    else:
        directory_path = os.path.abspath(directory)

    if not os.path.exists(directory_path):
        _LOGGER.warning("config_dir does not exist")
        os.mkdir(directory_path)

    return directory_path


def main() -> int:
    """Start Photos.network application."""
    validate_python()

    _LOGGER.debug("Now run the core system...")

    config_dir = get_or_create_directory(directory="config")
    data_dir = get_or_create_directory(directory="data", is_relative=False)

    runtime_conf = RuntimeConfig(
        config_dir=config_dir,
        data_dir=data_dir,
        safe_mode=False,
        debug=True,
        verbose=True,
    )

    try:
        exit_code = asyncio.run(setup_and_run(runtime_conf))
    except KeyboardInterrupt:
        # TODO: handle running threads
        print("### Interrupt application without taking care of running threads ###")
        exit_code = 0

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
