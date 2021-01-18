"""photo response"""


class PhotoResponse:
    def __init__(self, offset, limit, size, results):
        self.offset = offset
        self.limit = limit
        self.size = size
        self.results = results
