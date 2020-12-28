"""Exceptions used by webserver."""


class WebserverError(Exception):
    """General exception occurred."""


class Unauthorized(WebserverError):
    """When an action is unauthorized"""


class ServiceNotFound(WebserverError):
    """Raised when an error occured."""
