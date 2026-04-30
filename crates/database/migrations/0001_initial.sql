-- Accounts table must come first; media and references are owned by accounts.
CREATE TABLE IF NOT EXISTS accounts (
    account_id   VARCHAR PRIMARY KEY,
    email        VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    display_name VARCHAR,
    created_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS media (
    uuid         VARCHAR NOT NULL,
    owner        VARCHAR NOT NULL,
    name         VARCHAR,
    is_sensitive BOOLEAN DEFAULT FALSE,
    added_at     TIMESTAMPTZ DEFAULT NULL,
    taken_at     TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(owner) REFERENCES accounts(account_id)
);

CREATE TABLE IF NOT EXISTS reference (
    uuid          VARCHAR NOT NULL,
    media         VARCHAR NOT NULL,
    owner         VARCHAR NOT NULL,
    filepath      VARCHAR NOT NULL,
    filename      VARCHAR NOT NULL,
    size          INTEGER NOT NULL,
    description   VARCHAR DEFAULT NULL,
    last_modified TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media(uuid),
    FOREIGN KEY(owner) REFERENCES accounts(account_id)
);

CREATE TABLE IF NOT EXISTS details (
    uuid                VARCHAR NOT NULL,
    reference           VARCHAR DEFAULT NULL,
    camera_manufacturer VARCHAR DEFAULT NULL,
    camera_model        VARCHAR DEFAULT NULL,
    camera_serial       VARCHAR DEFAULT NULL,
    lens_model          VARCHAR DEFAULT NULL,
    lens_serial         VARCHAR DEFAULT NULL,
    orientation         VARCHAR DEFAULT NULL,
    compression         VARCHAR DEFAULT NULL,
    resolution_x        FLOAT DEFAULT NULL,
    resolution_y        FLOAT DEFAULT NULL,
    resolution_unit     VARCHAR DEFAULT NULL,
    exposure_time       FLOAT DEFAULT NULL,
    exposure_mode       VARCHAR DEFAULT NULL,
    exposure_program    VARCHAR DEFAULT NULL,
    exposure_bias       VARCHAR DEFAULT NULL,
    aperture            FLOAT DEFAULT NULL,
    focal_length        VARCHAR DEFAULT NULL,
    iso                 INTEGER NOT NULL,
    color_space         VARCHAR DEFAULT NULL,
    pixel_x             INTEGER NOT NULL,
    pixel_y             INTEGER NOT NULL,
    user_comment        VARCHAR DEFAULT NULL,
    white_balance       VARCHAR DEFAULT NULL,
    flash               BOOL DEFAULT NULL,
    exif_version        FLOAT DEFAULT NULL,
    FOREIGN KEY(reference) REFERENCES reference(uuid)
);

CREATE TABLE IF NOT EXISTS tags (
    uuid   VARCHAR NOT NULL,
    tag    VARCHAR NOT NULL,
    media  VARCHAR NOT NULL,
    origin VARCHAR DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media(uuid)
);

CREATE TABLE IF NOT EXISTS locations (
    uuid      VARCHAR NOT NULL,
    media     VARCHAR NOT NULL,
    latitude  FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    altitude  FLOAT DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media(uuid)
);
