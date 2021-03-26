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
        _LOGGER.debug("database file already exists.")
    else:
        _LOGGER.debug(f"create database {database_name}")
        _LOGGER.debug(f"database file_path {file_path}")
        file_object = open(file_path, "w")
        file_object.close()

    # return if setup was successful
    return True


async def create(core: ApplicationCore, key: str, value: str):
    _LOGGER.debug(f"add key {key} with value {value} into db")


async def read(core: ApplicationCore, key: str):
    # TODO: read value from database
    value = ""
    _LOGGER.debug(f"read key {key} from db. value = {value}")


async def update(core: ApplicationCore, key: str, value: str):
    # TODO: override key with value
    _LOGGER.debug(f"update key {key} in with value {value} db.")


async def delete(core: ApplicationCore, key: str):
    # remove key from db
    _LOGGER.debug(f"delete key {key} from db.")
