"""Next cloud integration to sync photos in a directory"""
import logging
import os
import shutil
import urllib
from typing import Dict
from urllib.request import urlretrieve

from core.addons.places365.main import generate_captions

from core.core import ApplicationCore

_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)

dir_places365_model = os.path.join(os.path.dirname(__file__), 'model')

filename_category = 'categories_places365.txt'
synset_url_category = 'https://raw.githubusercontent.com/csailvision/places365/master/categories_places365.txt'

filename_indoor = 'IO_places365.txt'
synset_url_indor = 'https://raw.githubusercontent.com/csailvision/places365/master/IO_places365.txt'

filename_sunattributes = 'labels_sunattribute.txt'
synset_url_sunattributes = 'https://raw.githubusercontent.com/csailvision/places365/master/labels_sunattribute.txt'

filename_wideresnet = 'W_sceneattribute_wideresnet18.npy'
synset_url_wideresnet = 'http://places2.csail.mit.edu/models_places365/W_sceneattribute_wideresnet18.npy'

filename_weights = 'wideresnet18_places365.pth.tar'
synset_url_weights = 'http://places2.csail.mit.edu/models_places365/wideresnet18_places365.pth.tar'


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup triggered by the ApplicationCore for each config entry."""

    try:
        check_or_download(filename=filename_category, download_url=synset_url_category)
        check_or_download(filename=filename_indoor, download_url=synset_url_indor)
        check_or_download(filename=filename_sunattributes, download_url=synset_url_sunattributes)
        check_or_download(filename=filename_wideresnet, download_url=synset_url_wideresnet)
        check_or_download(filename=filename_weights, download_url=synset_url_weights)
    except:
        _LOGGER.warning(f"Could not find or download all required files!")
        return False

    # Return boolean to indicate that initialization was successful.
    return True


async def async_process_images(core: ApplicationCore, config: dict, images: Dict) -> bool:
    for image_file in images:
        file_path = os.path.join(core.config.data_dir, image_file)
        captions = await generate_captions(file_path)

        # TODO: save environment, categories and attributes in database
        environment = captions['places365']['environment']
        categories = captions['places365']['categories']
        attributes = captions['places365']['attributes']

        _LOGGER.warning(f"environment {environment}")
        _LOGGER.warning(f"categories {categories}")
        _LOGGER.warning(f"attributes {attributes}")

    # TODO: check if all images processed without error
    return True


def check_or_download(filename: str, download_url: str) -> bool:
    """check if given filename exist inside 'models/' or try to download from given url."""
    file_path = os.path.join(dir_places365_model, filename)

    if not os.path.exists(dir_places365_model):
        os.mkdir(dir_places365_model)

    if not os.access(file_path, os.W_OK):
        _LOGGER.info(f"{file_path} not found, try to download from \n{download_url}")
        with urllib.request.urlopen(download_url) as response, open(file_path, 'wb') as out_file:
            shutil.copyfileobj(response, out_file)
        # urlretrieve(download_url, file_path)

    return True


