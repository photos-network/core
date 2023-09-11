CREATE TABLE IF NOT EXISTS users (
    --auto generated
    uuid VARCHAR NOT NULL,
    email VARCHAR,
    password VARCHAR,
    lastname VARCHAR,
    firstname VARCHAR,
    --indicates if user is not able to login
    is_locked BOOLEAN DEFAULT FALSE,
    create_at TIMESTAMPTZ DEFAULT NULL,
    updated_at TIMESTAMPTZ DEFAULT NULL,
    last_login TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS media (
    uuid VARCHAR NOT NULL,
    name VARCHAR,
    owner VARCHAR NOT NULL,
    -- show/hide by default regarding user settings
    is_sensitive BOOLEAN DEFAULT FALSE,
    -- added to photos.network
    date_added TIMESTAMPTZ DEFAULT NULL,
    -- captured timestamp
    date_taken TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(owner) REFERENCES users (uuid)
);

CREATE TABLE IF NOT EXISTS reference (
    uuid VARCHAR NOT NULL,
    media VARCHAR NOT NULL,
    -- ./data/files/[media.owner.uuid]/[media.date_taken.year]/[filename]
    filepath VARCHAR NOT NULL,
    -- original filename e.g. DSC1234.NEF
    filename VARCHAR NOT NULL,
    -- file size in bytes
    size INTEGER NOT NULL,
    description VARCHAR DEFAULT NULL,
    -- xmp, metadata
    last_modified TIMESTAMPTZ DEFAULT NULL,
    -- reference got deleted from 3rd party
    is_missing BOOLEAN DEFAULT TRUE,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media (uuid)
);
