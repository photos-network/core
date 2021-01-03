"""Metadata integration to parse image metadata like exif informations"""
import logging
import os
from typing import Dict

from exif import Image
from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup triggered by the ApplicationCore for each config entry."""
    _LOGGER.info(f"addon config: {config}")

    # Return boolean to indicate that initialization was successful.
    return True


async def async_process_images(core: ApplicationCore, config: dict, images: Dict) -> bool:
    for image_file in images:
        file_path = os.path.join(core.config.data_dir, image_file)
        with open(file_path, 'rb') as file:
            image = Image(file)

            msg = f"{file_path}: "

            if "datetime" in dir(image):
                datetime = image.get('datetime')
                msg += f"{datetime}"

            if "make" in dir(image) or "model" in dir(image):
                make = image.get('make')
                model = image.get('model')
                msg += f", {make} - {model}"

            if "gps_latitude" in dir(image) and "gps_longitude" in dir(image):
                latitude = image.get('gps_latitude')
                longitude = image.get('gps_longitude')

                msg += f", {latitude} - {longitude}"

            # TODO: persist metadata as tags?
            core.storage.create_key("metadata")

    return True
