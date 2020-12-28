"""Next cloud integration to sync photos in a directory"""
import logging


from core.core import ApplicationCore
from core.addons.places365.places365 import inference_places365

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup triggered by the ApplicationCore for each config entry."""

    # Return boolean to indicate that initialization was successful.
    return True


async def update(image_path: str) -> dict:
    captions = {}

    try:
        res_places365 = inference_places365(image_path)
        captions['places365'] = res_places365
        # self.captions_json = captions
        # if self.search_captions:
        #     self.search_captions = self.search_captions + ' , ' + \
        #     ' , '.join(res_places365['attributes'] + res_places365['categories'] + [res_places365['environment']])
        # else:
        #     self.search_captions = ' , '.join(
        #         res_places365['attributes'] + res_places365['categories'] +
        #         [res_places365['environment']])
        #
        # self.save()
        _LOGGER.info(f"generated places365 captions for image {image_path}.")
    except:
        _LOGGER.warning(f"could not generate places365 captions for image {image_path}")

    return captions
