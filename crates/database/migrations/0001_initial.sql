CREATE TABLE IF NOT EXISTS users (
    --auto generated
    uuid VARCHAR NOT NULL,
    email VARCHAR UNIQUE,
    password VARCHAR,
    lastname VARCHAR,
    firstname VARCHAR,
    displayname VARCHAR,
    --indicates if user is not able to login
    is_locked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NULL,
    updated_at TIMESTAMPTZ DEFAULT NULL,
    last_login_at TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid)
);

CREATE TABLE IF NOT EXISTS media (
    uuid VARCHAR NOT NULL,
    -- reference to `users`
    owner VARCHAR NOT NULL,
    name VARCHAR,
    -- show/hide by default regarding user settings
    is_sensitive BOOLEAN DEFAULT FALSE,
    -- added to photos.network
    added_at TIMESTAMPTZ DEFAULT NULL,
    -- captured timestamp
    taken_at TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(owner) REFERENCES users (uuid)
);

CREATE TABLE IF NOT EXISTS reference (
    uuid VARCHAR NOT NULL,
    -- reference to `media`
    media VARCHAR NOT NULL,
    -- reference to `users`
    owner VARCHAR NOT NULL,
    -- ./data/files/[media.owner.uuid]/[media.date_taken.year]/[filename]
    filepath VARCHAR NOT NULL,
    -- original filename e.g. DSC1234.NEF
    filename VARCHAR NOT NULL,
    -- file size in bytes
    size INTEGER NOT NULL,
    description VARCHAR DEFAULT NULL,
    -- xmp, metadata
    last_modified TIMESTAMPTZ DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media (uuid)
    FOREIGN KEY(owner) REFERENCES users (uuid)
);
CREATE TABLE IF NOT EXISTS details (
    uuid VARCHAR NOT NULL,
    -- reference to `reference`
    reference VARCHAR DEFAULT NULL,
    -- NIKON
    camera_manufacturer VARCHAR DEFAULT NULL,
    -- Z7
    camera_model VARCHAR DEFAULT NULL,
    -- 6014533
    camera_serial VARCHAR DEFAULT NULL,
    -- NIKKOR Z 35mm f/1.8 S
    lens_model VARCHAR DEFAULT NULL,
    -- 20028476
    lens_serial VARCHAR DEFAULT NULL,
    -- https://jdhao.github.io/2019/07/31/image_rotation_exif_info/
    orientation VARCHAR DEFAULT NULL,
    -- JPEG compression
    compression VARCHAR DEFAULT NULL,
    -- 72.0
    resolution_x FLOAT DEFAULT NULL,
    -- 72.0
    resolution_y FLOAT DEFAULT NULL,
    -- Inch
    resolution_unit VARCHAR DEFAULT NULL,
    -- 1/400 s
    exposure_time FLOAT DEFAULT NULL,
    -- Auto exposure
    exposure_mode VARCHAR DEFAULT NULL,
    -- Aperture priority
    exposure_program VARCHAR DEFAULT NULL,
    -- 0 EV
    exposure_bias VARCHAR DEFAULT NULL,
    -- 1.8
    aperture FLOAT DEFAULT NULL,
    focal_length VARCHAR DEFAULT NULL,
    iso INTEGER NOT NULL,
    -- sRGB
    color_space VARCHAR DEFAULT NULL,
    -- 8.256 pixel
    pixel_x INTEGER NOT NULL,
    -- 5.504 pixel
    pixel_y INTEGER NOT NULL,
    -- copyright info
    user_comment VARCHAR DEFAULT NULL,
    -- Auto white balance
    white_balance VARCHAR DEFAULT NULL,
    -- 0 = no flash
    flash BOOL DEFAULT NULL,
    -- Exif version 2.1
    exif_version FLOAT DEFAULT NULL,
    FOREIGN KEY(reference) REFERENCES reference (uuid)
);

CREATE TABLE IF NOT EXISTS tags (
    uuid VARCHAR NOT NULL,
    -- language unaware string like "landscape"
    tag VARCHAR NOT NULL,
    -- reference to `media`
    media VARCHAR NOT NULL,
    -- plugin name or `USER` where the tag comes from
    origin VARCHAR DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media (uuid)
);

CREATE TABLE IF NOT EXISTS locations (
    uuid VARCHAR NOT NULL,
    -- reference to `media`
    media VARCHAR NOT NULL,
    -- float gives a precision of ~1,7m
    -- see  https://stackoverflow.com/questions/159255/what-is-the-ideal-data-type-to-use-when-storing-latitude-longitude-in-a-mysql
    -- 48.13750
    latitude FLOAT NOT NULL,
    -- 11.57586
    longitude FLOAT NOT NULL,
    -- 520 m
    altitude FLOAT DEFAULT NULL,
    PRIMARY KEY (uuid),
    FOREIGN KEY(media) REFERENCES media (uuid)
);
