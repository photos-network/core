"""Auth client representation."""


class AuthClient:
    def __init__(
        self, client_name: str, client_id: str, client_secret: str, redirect_uris: dict
    ):
        self.client_name = client_name
        self.client_id = client_id
        self.client_secret = client_secret
        self.redirect_uris = redirect_uris

    def __repr__(self):
        return f"AuthClient(client_name={self.client_name!r}, redirect_uris={self.redirect_uris!r})"
