"""Metadata integration to parse image metadata like exif informations"""
import logging

from exif import Image
from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup triggered by the ApplicationCore for each config entry."""
    _LOGGER.info(f"addon config: {config}")

    # Return boolean to indicate that initialization was successful.
    return True


async def async_update(core: ApplicationCore, config: dict):
    # TODO: iterate through images for authenticated user
    # TODO: read exif data
    with open('grand_canyon.jpg', 'rb') as image_file:
        my_image = Image(image_file)
        has_exif = my_image.has_exif
        _LOGGER.error(f"metadata::async_update() has_exif: {has_exif}")
