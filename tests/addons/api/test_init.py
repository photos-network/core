"""Test the default api implementation."""
import asyncio
import os

import pytest

from core.core import ApplicationCore, CoreState
from core.webserver import status


@pytest.fixture
def mock_api_client():
    """Start the HTTP component and return admin API client."""
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)

    core = ApplicationCore()
    core.config.internal_url = ""
    core.config.external_url = ""
    core.config.data_dir = os.path.join(os.path.dirname(__file__), "resources", "data")
    core.state = CoreState.running
    core.start()

    return loop.run_until_complete(core)


async def test_api_get_non_existing_state(mock_api_client):
    """Test if the debug interface allows us to get a state."""
    resp = await mock_api_client.get("/api/states/does_not_exist")
    assert resp.status == status.HTTP_NOT_FOUND
