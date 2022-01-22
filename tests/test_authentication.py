"""Test."""
import aiohttp_jinja2
import jinja2
import pytest
from aiohttp import web
from aiohttp.test_utils import TestClient, TestServer

from core.authentication import Authentication, AuthenticationClient
from core.authentication.auth_database import AuthDatabase
from core.core import ApplicationCore


@pytest.mark.asyncio
async def test_authorization_grant(tmp_path):
    """test oauth authorization grant."""

    # Frontend as client_server
    async def handler(request):
        return web.Response(text="Frontend is running")

    core = ApplicationCore()
    core.config.config_dir = tmp_path
    core.config.data_dir = tmp_path

    client_app = web.Application()
    client_app.router.add_get("/", handler)
    client_server = TestServer(client_app)
    await client_server.start_server()
    redirect = f"http://{client_server.host}:{client_server.port}"

    # Core as authorization_server
    application = web.Application()
    aiohttp_jinja2.setup(
        application, loader=jinja2.FileSystemLoader("core/webserver/templates")
    )
    auth_database = AuthDatabase(core)
    auth = Authentication(core, application, auth_database)
    auth.add_client(
        AuthenticationClient(
            client_name="Frontend",
            client_id="a12b345c",
            client_secret="ABcD1E2F",
            redirect_uris=[redirect],
        )
    )
    authorization_server = TestServer(application)

    async with TestClient(authorization_server) as auth_client:
        await auth_client.start_server()

        get_resp = await auth_client.get(
            f"/api/oauth/authorize?client_id=a12b345c&response_type=code&redirect_uri={redirect}&scope=openid+profile+email+phone"
        )
        assert get_resp.status == 200
        text = await get_resp.text()
        assert "access the users public profile" in text

        resp = await auth_client.post(
            f"/api/oauth/authorize?client_id=a12b345c&response_type=code&redirect_uri={redirect}&scope=openid+profile+email+phone",
            data={"uname": "invalid", "password": "invalid"},
        )
        assert resp.status == 500
