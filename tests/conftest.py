"""Add helpers and fixtures for tests in this directory."""

import pytest
from aiohttp import web
from aiohttp.test_utils import TestServer


@pytest.fixture()
def mock_server(loop):
    app = web.Application()

    test_server = TestServer(app)
    return test_server
