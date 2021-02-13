"""Test the default api implementation."""
import os
from pathlib import Path

import pytest

from core.core import ApplicationCore, CoreState


@pytest.mark.asyncio
async def test_api_get_non_existing_state(aiohttp_client, loop):
    """Test if the debug interface allows us to get a state."""
    core = ApplicationCore()
    core.config.internal_url = ""
    core.config.external_url = ""
    core.config.data_dir = os.path.join(Path.cwd(), "tests/resources", "data")
    core.config.config_dir = os.path.join(Path.cwd(), "tests/resources", "config")
    core.state = CoreState.running
    await core.start()
