"""REST API implementation."""
import logging

from core.addons.api.auth import APIAuthView
from core.core import ApplicationCore, callback
from core.webserver.request import RequestView
from core.webserver.status import HTTP_CREATED, HTTP_OK

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """setup addon to core application."""
    is_cors_enabled = config["cors"]
    _LOGGER.info(f"enable cors: {is_cors_enabled}")

    core.http.register_request(APIStatusView())
    core.http.register_request(PhotoUploadView())
    core.http.register_request(PhotoView())
    core.http.register_request(AlbumView())
    core.http.register_request(APIAuthView())

    return True


class APIStatusView(RequestView):
    """View to handle Status requests."""

    requires_auth = False
    url = "/"
    name = "api:status"

    @callback
    def get(self, core, request):
        """Retrieve if API is running."""
        return self.json_message("API running.")


class PhotoUploadView(RequestView):
    """Upload new entities."""

    url = "/api/photo/"
    name = "api:photo:upload"

    async def post(self, request, entity_id):
        # TODO: check if entity has been created
        new_entity_created = False

        status_code = HTTP_CREATED if new_entity_created else HTTP_OK

        resp = self.json_message("Done", status_code)
        resp.headers.add("Location", f"/api/photo/{entity_id}")

        return resp


class PhotosView(RequestView):
    """View to handle photos requests."""

    url = "/api/photo"
    name = "api:photos"

    async def get(self, request):
        """d"""
        return self.json_message(f"return Photos")


class PhotoView(RequestView):
    """View to handle single photo requests."""

    url = "/api/photo/{entity_id}"
    name = "api:photo"

    async def get(self, request, entity_id):
        """d"""
        return self.json_message(f"return Photo {entity_id}")


class AlbumsView(RequestView):
    """View to handle albums requests."""

    requires_auth = False
    url = "/api/album"
    name = "api:albums"

    async def get(self, request):
        """Retrieve if API is running."""
        return self.json_message(f"return Albums")


class AlbumView(RequestView):
    """View to handle single album requests."""

    requires_auth = False
    url = "/api/album/{entity_id}"
    name = "api:album"

    async def get(self, request, entity_id):
        """Retrieve if API is running."""
        return self.json_message(f"return Album {entity_id}")
