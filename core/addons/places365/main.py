"""main entry point for places365."""
import logging

from core.addons.places365.places365 import inference_places365

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


async def generate_captions(image_path: str) -> dict:
    """Generate captions for given image_path with Places365CNN."""
    captions = {}

    try:
        res_places365 = inference_places365(image_path)
        captions["places365"] = res_places365

        _LOGGER.info(f"generated places365 captions for image {image_path}.")
    except:
        _LOGGER.warning(f"could not generate places365 captions for image {image_path}")

    return captions
