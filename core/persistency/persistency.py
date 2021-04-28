"""Persistency manager"""
import logging
import os
import time
from queue import Queue
from threading import Thread
from typing import TYPE_CHECKING, List, Optional, Set

from sqlalchemy import and_
from watchdog.events import FileSystemEvent, FileSystemEventHandler
from watchdog.observers import Observer

# Typing imports that create a circular dependency
from core.addon import AddonType
from core.configs import Config

from ..base import Session
from .dto.directory import Directory
from .dto.photo import Photo

if TYPE_CHECKING:
    from core.core import ApplicationCore


_LOGGER = logging.getLogger(__name__)
_LOGGER.setLevel(logging.DEBUG)


class FileHandler(FileSystemEventHandler):
    def __init__(self, queue: Queue):
        self.queue = queue

    def process(self, event: FileSystemEvent):
        self.queue.put(event)

    def on_created(self, event: FileSystemEvent):
        self.process(event)

    def on_moved(self, event):
        self.process(event)

    def on_deleted(self, event):
        self.process(event)


class PersistencyManager:
    """Class to manage persistency tasks and trigger storage addons."""

    def __init__(self, config: Config, core: "ApplicationCore") -> None:
        """Initialize Persistency Manager."""
        self.config = config
        self.core = core
        self.addons: Set[str] = config.addons
        self.observer = Observer()
        self.watchdog_queue = Queue()
        self.event_handler = FileHandler(self.watchdog_queue)

        # Set up a worker thread to process database load
        self.worker = Thread(
            target=self.process_load_queue, args=(self.watchdog_queue,)
        )
        self.worker.setDaemon(True)

    def process_load_queue(self, queue: Queue) -> None:
        """Process file/directory changes
        event.event_type
            'modified' | 'created' | 'moved' | 'deleted'
        event.is_directory
            True | False
        event.src_path
            path/to/observed/file
        """
        while True:
            if not queue.empty():
                event = queue.get()
                file = os.path.join(event.src_path)
                parent = os.path.dirname(file)
                filename = os.path.basename(file)

                if event.event_type == "created":
                    if not event.is_directory:
                        _LOGGER.info(f"added: {event.src_path}")
                        if os.path.exists(event.src_path):
                            directory = (
                                Session.query(Directory)
                                .filter(Directory.directory == parent)
                                .first()
                            )

                            # TODO: verify the filename does not exist already
                            photo = Photo(parent, filename, directory.owner)
                            Session.add(photo)
                            Session.commit()
                elif event.event_type == "moved":
                    _LOGGER.warning(f"file move not handled: {event.src_path}")
                elif event.event_type == "deleted":
                    Session.query(Photo).filter(
                        and_(Photo.directory == parent, Photo.filename == filename)
                    ).delete()
                    _LOGGER.info(f"deleted: {event.src_path}")
            else:
                time.sleep(1)

    async def start_directory_observing(self) -> None:
        """Start observing user directories for changes."""
        self.worker.start()

        directories = Session.query(Directory).all()
        for dir in directories:
            path = os.path.join(dir.directory)
            self.observer.schedule(self.event_handler, path, recursive=True)
        self.observer.start()

    async def stop_directory_observing(self) -> None:
        """Stop observing user directories."""
        self.worker.stop()
        self.observer.stop()
        self.observer.join()

    async def restart_directory_observing(self) -> None:
        """stop and re-start directory observing."""
        await self.stop_directory_observing()
        await self.start_directory_observing()

    def create_user_home(self, user_id: str) -> None:
        """Create a directory for the user in the configured data directory."""
        # create user_home in filesystem
        path = os.path.join(self.config.data_dir, user_id)
        os.mkdir(path)

        # add user_home to observing
        self.observer.schedule(self.event_handler, path, recursive=True)

        # add user_home to database
        directory = Directory(path, user_id)
        Session.add(directory)
        Session.commit()

        _LOGGER.info(f"user home for user {user_id} created.")

    async def read_photos(self, user_id: int, offset: int = 0, limit: int = 10) -> List:
        return (
            Session.query(Photo)
            .filter(Photo.owner == user_id)
            .offset(offset * limit)
            .limit(limit)
            .all()
        )

    async def read_photo(self, photo_id) -> Optional[Photo]:
        """request a single photo by id from database"""
        return Session.query(Photo).filter(Photo.uuid == photo_id).first()

    async def scan_user_directory(self, directory) -> None:
        """Scan given directory and add media items into the database."""
        pass

    async def file_exist(self, file_name: str, path: str = None) -> bool:
        if path is None:
            path = self.config.data_dir

        _LOGGER.info(f"check if file {file_name} exists in path {path}")

        return os.path.exists(os.path.join(file_name, path))

    async def create_file(self, file_name: str, path: str = None):
        if path is None:
            default_path = self.config.data_dir
        else:
            default_path = path

        _LOGGER.info(f"create file {file_name} at {default_path}")

        file_object = open(os.path.join(file_name, default_path), "x")
        file_object.close()

    async def create(self, key, value):
        """
        Create the given key/value pair to all storage addons
        """
        for addon in self.core.loaded_addons.values():
            if addon.type == AddonType.STORAGE:
                # TODO: write key to persistency
                _LOGGER.warning(f"  Addon: {addon} {type(addon)} {key}")
                await addon.create(key, value)

    async def read(self, key):
        """
        Try to read the given key from all storage addons.
        """
        for addon in self.core.loaded_addons.values():
            if addon.type == AddonType.STORAGE:
                # TODO: read key from persistency

                _LOGGER.warning(
                    f"  Addon: {addon} {type(self.core.loaded_addons)} {key}"
                )
                _LOGGER.warning(f"  Addon: {addon} {type(addon)} {key}")
                await addon.read(key)

    async def update(self, key: str, value: str):
        for addon in self.core.loaded_addons.values():
            if addon.type == AddonType.STORAGE:
                # TODO: write key to persistency
                _LOGGER.warning(f"  Addon: {addon} {type(addon)} {key}")
                await addon.update(key, value)

    async def delete(self, key: str):
        for addon in self.core.loaded_addons.values():
            if addon.type == AddonType.STORAGE:
                # TODO: write key to persistency
                _LOGGER.warning(f"  Addon: {addon} {type(addon)} {key}")
                await addon.delete(key)
