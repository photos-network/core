"""REST API implementation."""
import json
import logging
import os
import pathlib
from datetime import datetime
from typing import Optional

from aiohttp import web
from core.addons.api.dto.details import Details
from core.addons.api.dto.location import Location
from core.addons.api.dto.photo import (PhotoDetailsResponse, PhotoEncoder,
                                       PhotoResponse)
from core.addons.api.dto.photo_response import PhotosResponse
from core.base import Session
from core.core import ApplicationCore
from core.persistency.dto.photo import Photo
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
    core.http.register_request(PhotoDetailsView())
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

    url = "/v1/photos"
    name = "v1:photo"

    async def get(self, core: ApplicationCore, request: web.Request) -> web.Response:
        """Get a list of all photo resources."""
        _LOGGER.debug(f"GET /v1/photos")
        await core.authentication.check_permission(request, "library:read")

        user_id = await core.http.get_user_id(request)

        if user_id is None:
            raise web.HTTPForbidden()

        limit = 50
        if "limit" in request.query:
            limit = int(request.query["limit"])

        offset = 0
        if "offset" in request.query:
            offset = int(request.query["offset"])

        _LOGGER.debug(f"read {limit} photos for user_id {user_id} beginning with {offset}")
        user_photos = await core.storage.read_photos(user_id, offset, limit)

        results = []

        for photo in user_photos:
            results.append(
                PhotoResponse(
                    id=photo.uuid,
                    name=photo.filename,
                    image_url=f"{core.config.external_url}/v1/file/{photo.uuid}"
                )
            )

        response = PhotosResponse(offset=offset, limit=limit, size=len(results), results=results)
        return web.Response(text=json.dumps(response, cls=PhotoEncoder), content_type="application/json")


class PhotoDetailsView(RequestView):
    """View to handle single photo requests."""

    url = "/v1/photo/{entity_id}"
    name = "v1:photo"

    async def get(self, core: ApplicationCore, request: web.Request, entity_id: str) -> web.Response:
        """Return an entity."""
        _LOGGER.debug(f"GET /v1/photo/{entity_id}")

        # TODO: add user_id to check if user has access to image
        photo = await core.storage.read_photo(entity_id)

        if photo is None:
            raise web.HTTPNotFound

        # photo owner
        # TODO: get first-/lastname of owner
        # owner = await core.authentication

        _LOGGER.debug(f"photo {photo.uuid}")
        file = os.path.join(photo.directory, photo.filename)
        _LOGGER.debug(f"get additional data for {file} / {os.path.exists(file)}")

        # photo creation time
        fname = pathlib.Path(file)
        mtime = datetime.fromtimestamp(fname.stat().st_mtime)
        ctime = datetime.fromtimestamp(fname.stat().st_ctime)

        # photo location
        location = None
        latitude = await core.storage.read("latitude")
        longitude = await core.storage.read("longitude")
        if latitude is not None and longitude is not None:
            altitude = await core.storage.read("altitude")
            if altitude is not None:
                location = Location(latitude=latitude, longitude=longitude, altitude=altitude)
            else:
                location = Location(latitude=latitude, longitude=longitude, altitude="0.0")

        # photo tags
        tags = await core.storage.read("tags")

        result = PhotoDetailsResponse(
            id=photo.uuid,
            name=photo.filename,
            author=photo.owner,
            created_at=ctime.isoformat(),
            details=Details(
                        camera="Nikon Z7",
                        lens="Nikkor 200mm F1.8",
                        focal_length="200",
                        iso="400",
                        shutter_speed="1/2000",
                        aperture="4.0",
            ),
            tags=tags,
            location=location,
            image_url=f"{core.config.external_url}/v1/file/{entity_id}"
        )
        return web.Response(text=json.dumps(result, cls=PhotoEncoder), content_type="application/json")


class PhotoView(RequestView):
    """View to handle photo file requests."""

    # TODO: enable auth
    requires_auth = False
    url = "/v1/file/{entity_id}"
    name = "v1:file"

    async def get(self, core: ApplicationCore, request: web.Request, entity_id: str) -> web.Response:
        """Return an entity."""
        _LOGGER.debug(f"GET /v1/file/{entity_id}")

        # TODO: parse params  max-with / max-height   =wmax-width-hmax-height  (=w2048-h1024)
        # -wmax-width   (preserving the aspect ratio)
        # -hmax-height  (preserving the aspect ratio)
        # -c  crop images to max-width / max-height
        # -d  remove exif data

        result = Session.query(Photo).filter(Photo.uuid == entity_id).first()

        file = os.path.join(result.directory, result.filename)
        if os.path.exists(os.path.join(file)):
            return web.FileResponse(path=file, status=200)
        else:
            raise web.HTTPNotFound()

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

        resp = self.json_message(f"File successfully added with ID: {new_entity_id}", status_code)
        resp.headers.add("Location", f"/api/photo/{new_entity_id}")

        return resp

    async def delete(self, core: ApplicationCore, request: web.Request, entity_id: str):
        """Delete an entity."""
        _LOGGER.debug(f"DELETE /v1/file/{entity_id}")

        # TODO: delete entity
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
