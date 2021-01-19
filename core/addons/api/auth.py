"""Authentication flow implementation."""
import json
from json import JSONEncoder

from aiohttp import web

from core.webserver import status
from core.webserver.request import RequestView
from core.webserver.type import APPLICATION_JSON


class APIAuthView(RequestView):
    """View to handle auth requests."""

    requires_auth = False
    url = "/auth"
    name = "api:auth"

    async def post(self):
        """Handle authentication requests."""
        # TODO: handle data for authentication
        msg = json.dumps(
            "Authentication not implemented yet!", cls=JSONEncoder, allow_nan=False
        ).encode("UTF-8")

        response = web.Response(
            body=msg,
            content_type=APPLICATION_JSON,
            status=status.HTTP_UNPROCESSABLE_ENTITY,
        )
        response.enable_compression()
        return response
