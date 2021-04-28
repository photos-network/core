import uuid

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import scoped_session, sessionmaker

# TODO: get data directory from config
engine = create_engine("sqlite:///data/system.sqlite3", echo=False)
session_factory = sessionmaker(bind=engine)
Session = scoped_session(session_factory)

Base = declarative_base()


def generate_uuid():
    return str(uuid.uuid4())
