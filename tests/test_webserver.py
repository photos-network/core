"""Test webserver implementation."""
from aiohttp import web


async def hello(request):
    """Test webserver request."""
    return web.Response(text="Hello, world")


async def test_hello(aiohttp_client, loop):
    """Test webserver aiohttp client."""
    app = web.Application()
    app.router.add_get("/", hello)
    client = await aiohttp_client(app)
    resp = await client.get("/")
    assert resp.status == 200
    text = await resp.text()
    assert "Hello, world" in text
