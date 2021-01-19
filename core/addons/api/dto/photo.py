"""Photo."""
from json.encoder import JSONEncoder


class PhotoEncoder(JSONEncoder):
    """Encode photo to json."""

    def default(self, o):
        """Encode all properties."""
        return o.__dict__


class Photo:
    """Photo response object."""

    def __init__(
        self, name, description, author, created_at, details, tags, location, image_urls
    ):
        """Initialize photo response object."""
        self.name = name
        self.description = description
        self.author = author
        self.created_at = created_at
        self.details = details
        self.tags = tags
        self.location = location
        self.image_urls = image_urls
