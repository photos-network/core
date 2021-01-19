"""Core application component."""
import asyncio
import enum
import json
import logging
import os
import sys
from logging.handlers import TimedRotatingFileHandler
from time import monotonic
from typing import (
    Any,
    Awaitable,
    Callable,
    Dict,
    Iterable,
    List,
    Optional,
    Sequence,
    TypeVar,
)

from colorlog import ColoredFormatter

from core import loader
from core.addon import Addon
from core.configs import Config
from core.persistency.persistency import PersistencyManager
from core.utils.timeout import TimeoutManager
from core.webserver import Webserver

ERROR_LOG_FILENAME = "core.log"

# core.data key for logging information.
DATA_LOGGING = "logging"

# How long to wait to log tasks that are blocking
BLOCK_LOG_TIMEOUT = 60  # seconds

_LOGGER = logging.getLogger(__name__)
logging.basicConfig(level=logging.DEBUG)

CALLABLE_T = TypeVar("CALLABLE_T", bound=Callable)


def callback(func: CALLABLE_T) -> CALLABLE_T:
    """Annotation to mark method as safe to call from within the event loop."""
    setattr(func, "_callback", True)
    return func


class ApplicationCore:
    """ApplicationCore root object."""

    http: "Webserver" = None  # type: ignore

    def __init__(self) -> None:
        """Initialize new ApplicationCore."""
        _LOGGER.debug("ApplicationCore::init...")

        self.loop = asyncio.get_running_loop()
        self._pending_tasks: List = []
        self._track_task = True
        self.banned_ips: List = []
        self.failed_logins: Dict = {}
        self.config = Config(self)
        self.addons = loader.Components(self)
        self.loaded_addons: Dict = {}

        # This is a dictionary that any addon can store any data on.
        self.data: dict = {}
        self.storage: PersistencyManager = PersistencyManager(self.config, self)
        self.state: CoreState = CoreState.not_running
        self.exit_code: int = 0

        # If not None, use to signal end-of-loop
        self._stopped: Optional[asyncio.Event] = None

        # Timeout handler for Core/Helper namespace
        self.timeout: TimeoutManager = TimeoutManager()

    @property
    def is_running(self) -> bool:
        """Return if Photos Core is running."""
        return self.state in (CoreState.starting, CoreState.running)

    @property
    def is_stopping(self) -> bool:
        """Return if Photos Core is stopping."""
        return self.state in (CoreState.stopping, CoreState.final_write)

    def async_enable_logging(self, verbose: bool = False) -> None:
        """Set up logging for application core."""
        fmt = "%(asctime)s %(levelname)s (%(threadName)s) [%(name)s] %(message)s"
        datefmt = "%Y-%m-%d %H:%M:%S"
        logging.basicConfig(level=logging.INFO)

        colorfmt = f"%(log_color)s{fmt}%(reset)s"
        logging.getLogger().handlers[0].setFormatter(
            ColoredFormatter(
                colorfmt,
                datefmt=datefmt,
                reset=True,
                log_colors={
                    "DEBUG": "cyan",
                    "INFO": "green",
                    "WARNING": "yellow",
                    "ERROR": "red",
                    "CRITICAL": "red",
                },
            )
        )

        logging.basicConfig(format=fmt, datefmt=datefmt, level=logging.INFO)

        logging.getLogger("requests").setLevel(logging.WARNING)
        logging.getLogger("urllib3").setLevel(logging.WARNING)
        logging.getLogger("aiohttp.access").setLevel(logging.WARNING)

        sys.excepthook = lambda *args: logging.getLogger("").exception(
            "Uncaught exception", exc_info=args  # type: ignore
        )
        log_rotate_days = 14
        err_log_path = self.config.path(ERROR_LOG_FILENAME)
        err_dir = os.path.dirname(err_log_path)
        if not err_dir:
            os.mkdir(err_dir)
        err_handler: logging.FileHandler = TimedRotatingFileHandler(
            err_log_path, when="midnight", backupCount=log_rotate_days
        )
        err_handler.setLevel(logging.DEBUG if verbose else logging.WARNING)
        err_handler.setFormatter(logging.Formatter(fmt, datefmt=datefmt))

        logger = logging.getLogger("")
        logger.addHandler(err_handler)
        logger.setLevel(logging.DEBUG if verbose else logging.WARNING)

    async def start(self) -> int:
        """Start Photos.network application core.

        Note: This function is only used for testing.
        For regular use, use "await photos.run()".
        """
        _LOGGER.debug("start core loop")
        await self.async_block_till_done()

        return self.exit_code

    async def async_run(self, *, attach_signals: bool = True) -> int:
        """Run the application.

        Main entry point of the core application.
        """
        _LOGGER.debug("ApplicationCore::async_run")

        if self.state != CoreState.not_running:
            raise RuntimeError("Core is already running")

        # _async_stop will set this instead of stopping the loop
        self._stopped = asyncio.Event()

        await self.async_start()

        await self._stopped.wait()
        return self.exit_code

    async def async_start(self) -> None:
        """Finalize startup from inside the event loop."""
        _LOGGER.debug("ApplicationCore::async_start()")

        self.state = CoreState.starting

        try:
            await self.async_block_till_done()
        except asyncio.TimeoutError:
            _LOGGER.warning(
                "Something is blocking core from wrapping up the "
                "start up phase. We're going to continue anyway."
            )

        # Wait for all startup triggers before changing state
        await asyncio.sleep(0)

        if self.state != CoreState.starting:
            _LOGGER.warning(
                "Photos.network startup has been interrupted. "
                "Its state may be inconsistent"
            )
            return

        self.state = CoreState.running

    async def async_stop(self) -> None:
        """Stop Photos.network core application."""
        self.state = CoreState.stopping
        self.state = CoreState.final_write
        self.state = CoreState.not_running
        self.state = CoreState.stopped

        if self._stopped is not None:
            self._stopped.set()

    async def async_set_up_addons(self) -> None:
        """Set up addons by checking and installing their dependencies and run their 'async_setup' method."""
        conf_dict = await self._load_config()
        addon_dict = await self._load_addons(conf_dict)

        for addon in addon_dict.values():
            # _LOGGER.info(f"setup addon '{addon.domain}'")

            all_requirements_fulfilled = addon.install_requirements()
            if not all_requirements_fulfilled:
                _LOGGER.error(
                    f"setup addon '{addon.domain}' failed. Not all requirements installed!"
                )

            if all_requirements_fulfilled:
                addon_setup_successful = await addon.async_setup_addon()
                if addon_setup_successful:
                    self.loaded_addons[addon.domain] = addon

    async def async_trigger_addons(self, images: Sequence[str]) -> None:
        """Trigger processing of all image addons."""
        conf_dict = await self._load_config()
        addon_dict = await self._load_addons(conf_dict)

        for addon in addon_dict.values():
            await addon.async_process_images_in_addons(images)

    async def _load_addons(self, conf_dict) -> Dict[str, Addon]:
        """Load addons from files."""
        addons = {}
        for item in conf_dict.get("addons"):
            addon_name = item.get("name")
            addon_config = item.get("config")
            addon = Addon.resolve_from_root(self, addon_name, addon_config)
            if addon is not None:
                addons[addon_name] = addon

        return addons

    async def _load_config(self) -> dict:
        """Load configuration from file and return as dict."""
        config_file = os.path.join(self.config.config_dir, "configuration.json")
        _LOGGER.info(f"config_file {config_file}")
        with open(config_file, encoding="utf-8") as file:
            conf_dict = json.load(file)

        if not isinstance(conf_dict, dict):
            msg = (
                f"The configuration file {os.path.basename(self.config.config_dir)} "
                "does not contain a dictionary"
            )
            _LOGGER.error(msg)
            raise RuntimeError(msg)
        return conf_dict

    async def async_block_till_done(self) -> None:
        """Block until all pending work is done."""
        # To flush out any call_soon_threadsafe
        await asyncio.sleep(0)
        start_time: Optional[float] = None

        self.http = Webserver(self)

        # setup addons from config entries
        await self.async_set_up_addons()

        await self.http.start()
        _LOGGER.info("Webserver should be up and running...")

        while self._pending_tasks:
            pending = [task for task in self._pending_tasks if not task.done()]
            self._pending_tasks.clear()
            if pending:
                await self._await_and_log_pending(pending)

                if start_time is None:
                    # Avoid calling monotonic() until we know
                    # we may need to start logging blocked tasks.
                    start_time = 0
                elif start_time == 0:
                    # If we have waited twice then we set the start time
                    start_time = monotonic()
                elif monotonic() - start_time > BLOCK_LOG_TIMEOUT:
                    # We have waited at least three loops and new tasks
                    # continue to block. At this point we start
                    # logging all waiting tasks.
                    for task in pending:
                        _LOGGER.debug("Waiting for task: %s", task)
            else:
                await asyncio.sleep(0)

    async def _await_and_log_pending(self, pending: Iterable[Awaitable[Any]]) -> None:
        """Await and log tasks that take a long time."""
        wait_time = 0
        while pending:
            _, pending = await asyncio.wait(pending, timeout=BLOCK_LOG_TIMEOUT)
            if not pending:
                return
            wait_time += BLOCK_LOG_TIMEOUT
            for task in pending:
                _LOGGER.debug("Waited %s seconds for task: %s", wait_time, task)


class CoreState(enum.Enum):
    """Represent the current state of Photos.network."""

    not_running = "NOT_RUNNING"
    starting = "STARTING"
    running = "RUNNING"
    stopping = "STOPPING"
    final_write = "FINAL_WRITE"
    stopped = "STOPPED"

    def __str__(self) -> str:  # pylint: disable=invalid-str-returned
        """Return the event."""
        return self.value  # type: ignore
