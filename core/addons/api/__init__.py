"""REST API implementation."""
import json
import logging
from json import JSONEncoder

from aiohttp import web

from core.addons.api.auth import APIAuthView
from core.webserver import status
from core.core import ApplicationCore, callback
from core.webserver.request import Request
from core.webserver.type import APPLICATION_JSON

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup addon to core application."""
    is_cors_enabled = config["cors"]
    _LOGGER.info(f"enable cors: {is_cors_enabled}")
    core.http.register_request(APIStatusView)
    core.http.register_request(APIAuthView)

    return True


class APIStatusView(Request):
    """View to handle Status requests."""

    requires_auth = False
    url = "/"
    name = "api:status"

    async def get(self):
        """Retrieve if API is running."""
        msg = json.dumps("API is running", cls=JSONEncoder, allow_nan=False).encode("UTF-8")

        response = web.Response(
            body=msg,
            content_type=APPLICATION_JSON,
            status=status.HTTP_OK,
        )
        response.enable_compression()
        return response
