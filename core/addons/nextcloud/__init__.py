"""Next cloud integration to sync photos in a directory."""
import logging

from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup triggered by the ApplicationCore for each config entry."""
    _LOGGER.info(f"addon config: {config}")

    # TODO: instantiation should be done per each user instead
    # for entry in config:
    #     _LOGGER.debug(f'config: {entry}')
    #     server = entry.get("server")
    #     port = entry.get("port")
    #     username = entry.get("username")
    #     password = entry.get("password")

    # Return boolean to indicate that initialization was successful.
    return True
