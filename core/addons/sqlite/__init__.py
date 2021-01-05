"""sqlite implementation to store meta data in a database."""
import logging
import os

from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup addon to core application."""

    _LOGGER.debug(f"config: {config}")
    _LOGGER.debug(f'database file: {config["database_file"]}')

    if "database_file" not in config:
        _LOGGER.warning("could not find database_file in configuration!")
        database_name = "database.sqlite"
    else:
        database_name = config["database_file"]

    file_path = os.path.join(core.config.data_dir, database_name)

    if os.path.exists(os.path.abspath(file_path)):
        _LOGGER.info("database file already exists.")
    else:
        _LOGGER.info(f"create database {database_name}")
        file_object = open(file_path, "w")
        file_object.close()

    return True


async def create(key: str):
    _LOGGER.debug(f"add key {key} into db")


async def read(key: str):
    _LOGGER.debug(f"read key {key} into db")


async def update(key: str):
    _LOGGER.debug(f"update key {key} in db")


async def delete(key: str):
    _LOGGER.debug(f"delete key {key} from db")
