"""REST API implementation."""
import json
import logging
import os

from aiohttp import web

from core.addons.api.dto.details import Details
from core.addons.api.dto.location import Location
from core.addons.api.dto.photo import Photo, PhotoEncoder
from core.addons.api.dto.photo_response import PhotoResponse
from core.addons.api.dto.photo_url import PhotoUrl
from core.core import ApplicationCore
from core.webserver.request import KEY_USER_ID, RequestView
from core.webserver.status import HTTP_CREATED, HTTP_OK

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)


async def async_setup(core: ApplicationCore, config: dict) -> bool:
    """Set up addon to core application."""
    if config is not None and "cors" in config:
        is_cors_enabled = config["cors"]
    else:
        is_cors_enabled = False
    _LOGGER.debug(f"enable cors: {is_cors_enabled}")

    core.http.register_request(APIStatusView())
    core.http.register_request(PhotosView())
    core.http.register_request(PhotoView())
    core.http.register_request(AlbumView())

    return True


class APIStatusView(RequestView):
    """View to handle Status requests."""

    requires_auth = False
    url = "/"
    name = "api:status"

    async def get(self, core: ApplicationCore, request: web.Request):
        """Retrieve if API is running."""
        return self.json_message("API running.")

    async def head(self, core: ApplicationCore, request: web.Request):
        """Retrieve if API is running."""
        return self.json_message("API running.")


class PhotosView(RequestView):
    """View to handle photos requests."""

    url = "/api/photo/"
    name = "api:photo:upload"

    async def get(self, core: ApplicationCore, request: web.Request) -> web.Response:
        """Get a list of all photo resources."""
        user_id = await core.http.get_user_id(request)

        if user_id is None:
            raise web.HTTPForbidden()

        _LOGGER.debug(f"read photos for user_id {user_id}")
        user_photos = await core.storage.read_photos(user_id)
        _LOGGER.debug(f"iterate through {len(user_photos)} photos.")

        results = []

        # iterate through photos
        for photo in user_photos:
            _LOGGER.debug(f"get additional data for {photo.filepath}")

            # photo location
            location = None
            latitude = await core.storage.read("latitude")
            longitude = await core.storage.read("longitude")
            if latitude is not None and longitude is not None:
                altitude = await core.storage.read("altitude")
                if altitude is not None:
                    location = Location(
                        latitude=latitude, longitude=longitude, altitude=altitude
                    )
                else:
                    location = Location(
                        latitude=latitude, longitude=longitude, altitude="0.0"
                    )

            # photo tags
            tags = await core.storage.read("tags")
            tags = ["landscape", "sky", "night"]

            # add photo to results
            results.append(
                Photo(
                    name="DSC_2340-HDR.jpg",
                    description="",
                    author="",
                    created_at="2012-02-09T21:11:53-05:00",
                    details=Details(
                        camera="Canon EOS-1D Mark IV",
                        lens="E 18-200mm F3.5-6.3 OSS",
                        focal_length="700",
                        iso="400",
                        shutter_speed="1/2000",
                        aperture="6.3",
                    ),
                    tags=tags,
                    location=location,
                    image_urls=[
                        PhotoUrl(size="1080", url="/data/cache/DSC_2340-HDR_1080.jpg"),
                        PhotoUrl(size="1600", url="/data/cache/DSC_2340-HDR_1600.jpg"),
                        PhotoUrl(size="2048", url="/data/cache/DSC_2340-HDR_2048.jpg"),
                        PhotoUrl(size="full", url="/data/cache/DSC_2340-HDR.jpg"),
                    ],
                )
            )

        # key = "latitude"
        # _LOGGER.error(f"key/value: {key}/{value}")

        # data = request.query

        offset = 0
        # if data["size"]:
        #     offset = data["size"]  # integer  0..N

        limit = 50
        # if data["size"]:  # integer   Number of records per page.
        #     limit = data["size"]

        _LOGGER.info(f"loading data for user {user_id}")

        response = PhotoResponse(
            offset=offset, limit=limit, size=len(results), results=results
        )

        return web.Response(
            text=json.dumps(response, cls=PhotoEncoder), content_type="application/json"
        )

    async def post(self, core: ApplicationCore, request: web.Request) -> web.Response:
        """Upload new photo resource."""
        user = request[KEY_USER_ID]

        reader = await request.multipart()

        field = await reader.next()
        assert field.name == "data"
        original_filename = field.filename

        _LOGGER.warning(f"request: {original_filename}")

        # TODO: get storage directory for user
        path = os.path.join(f"./data/users/{user}/", original_filename)
        if os.path.exists(path):
            # TODO: handle filename already exists
            _LOGGER.warning(f"file already exists! {path}")
        filename = original_filename

        size = 0
        with open(os.path.join(f"./data/users/{user}/", filename), "wb") as f:
            while True:
                chunk = await field.read_chunk()  # 8192 bytes by default.
                if not chunk:
                    break
                size += len(chunk)
                f.write(chunk)

        _LOGGER.error(f"added {filename} to data directory of {user}.")

        # TODO: add new filepath to database
        # TODO: return unique_id from database
        new_entity_id = 123456

        new_entity_created = True

        status_code = HTTP_CREATED if new_entity_created else HTTP_OK

        resp = self.json_message(
            f"File successfully added with ID: {new_entity_id}", status_code
        )
        resp.headers.add("Location", f"/api/photo/{new_entity_id}")

        return resp


class PhotoView(RequestView):
    """View to handle single photo requests."""

    url = "/api/photo/{entity_id}"
    name = "api:photo"

    async def get(self, request: web.Request, entity_id) -> web.Response:
        """Return an entity."""
        return self.json_message(f"return GET {entity_id}")

    async def post(self, request: web.Request, entity_id):
        """Create an entity."""
        return self.json_message(f"return POST {entity_id}")

    async def delete(self, request: web.Request, entity_id):
        """Delete an entity."""
        return self.json_message(f"return DELETE {entity_id}")


class AlbumsView(RequestView):
    """View to handle albums requests."""

    requires_auth = False
    url = "/api/album/"
    name = "api:albums"

    async def get(self, request: web.Request) -> web.Response:
        """Retrieve if API is running."""
        return self.json_message("return Albums")


class AlbumView(RequestView):
    """View to handle single album requests."""

    requires_auth = False
    url = "/api/album/{entity_id}"
    name = "api:album"

    async def get(self, request: web.Request, entity_id) -> web.Response:
        """Retrieve if API is running."""
        return self.json_message(f"return Album {entity_id}")

    async def post(self, request: web.Request) -> web.Response:
        """Create a new album."""
        return self.json_message("create a new Album")
