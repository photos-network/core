"""Constants used by Photos.network core."""
MAJOR_VERSION = 0
MINOR_VERSION = 3
PATCH_VERSION = 0
__short_version__ = f"{MAJOR_VERSION}.{MINOR_VERSION}"
__version__ = f"{__short_version__}.{PATCH_VERSION}"
REQUIRED_PYTHON_VER = (3, 7, 1)

# The exit code to send to request a restart
RESTART_EXIT_CODE = 100

DEVICE_DEFAULT_NAME = "Unnamed Device"

URL_API = "/api"

# token lifetime: 1hour (60sec * 60min)
CONF_TOKEN_LIFETIME = 3600
CONF_ACCESS_TOKEN = "access_token"
CONF_ADDRESS = "address"
CONF_CLIENT_ID = "client_id"
CONF_CLIENT_SECRET = "client_secret"

# token lifetime: 30days (60sec * 60min * 24hour * 30days)
CONF_DEADLINE = 2592000
CORE_VERSION = __version__
