"""Test."""
import aiohttp_jinja2
import jinja2
import pytest
from aiohttp import web
from aiohttp.test_utils import TestClient, TestServer

from core.authentication import Auth


@pytest.mark.asyncio
async def test_authorization_grant():
    """test oauth authorization grant."""

    async def hello(request):
        return web.Response(text="Hello, world")

    client_server = web.Application()
    client_server.router.add_get("/", hello)
    client_server2 = TestServer(client_server)

    authorization_server = web.Application()
    aiohttp_jinja2.setup(
        authorization_server, loader=jinja2.FileSystemLoader("core/webserver/templates")
    )
    Auth(authorization_server)

    test_server = TestServer(authorization_server)

    async with TestClient(test_server) as auth_client:
        await auth_client.start_server()

        async with TestClient(client_server2) as app_client:
            await app_client.start_server()

            redirect = f"http://{app_client.host}:{str(app_client.port)}"
            get_resp = await auth_client.get(
                f"/oauth/authorize?client_id=Client_ID&redirect_uri={redirect}&scope=openid+profile+email+phone"
            )
            assert get_resp.status == 200
            text = await get_resp.text()
            assert "access the users public profile" in text

            resp = await auth_client.post(
                "/oauth/authorize",
                data={"uname": "testuser", "password": "testpass"},
            )
            assert resp.status == 200
