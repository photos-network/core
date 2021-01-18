"""photo"""
from json.encoder import JSONEncoder


class PhotoEncoder(JSONEncoder):
    def default(self, o):
        return o.__dict__


class Photo:
    def __init__(self, name, description, author, created_at, details, tags, location, image_urls):
        self.name = name
        self.description = description
        self.author = author
        self.created_at = created_at
        self.details = details
        self.tags = tags
        self.location = location
        self.image_urls = image_urls