"""Photo."""
from json.encoder import JSONEncoder


class PhotoEncoder(JSONEncoder):
    """Encode photo to json."""

    def default(self, o):
        """Encode all properties."""
        return o.__dict__


class PhotoResponse:
    """Photo response object."""

    def __init__(self, id, name, image_url, date_added, date_taken):
        """Initialize photo response object."""
        self.id = id
        self.name = name
        self.image_url = image_url
        self.date_added = date_added
        self.date_taken = date_taken


class PhotoDetailsResponse:
    """Photo response object."""

    def __init__(
        self,
        id,
        name,
        owner,
        date_added,
        date_taken,
        details,
        tags,
        location,
        image_url,
    ):
        """Initialize photo response object."""
        self.id = id
        self.name = name
        self.owner = owner
        self.date_added = date_added
        self.date_taken = date_taken
        self.details = details
        self.tags = tags
        self.location = location
        self.image_url = image_url
