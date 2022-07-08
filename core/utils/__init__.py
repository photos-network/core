"""utils and helpers"""
import random
import string


def random_string(length: int) -> str:
    return "".join(random.choices(string.ascii_uppercase + string.ascii_lowercase + string.digits, k=length))


def generate_random_client_id() -> str:
    return (
        random_string(8)
        + "-"
        + random_string(4)
        + "-"
        + random_string(4)
        + "-"
        + random_string(4)
        + "-"
        + random_string(12)
    )


def generate_random_client_secret() -> str:
    return random_string(22)
