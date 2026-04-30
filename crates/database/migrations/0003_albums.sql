-- Albums owned by a photographer (user)
CREATE TABLE IF NOT EXISTS albums (
    album_id       VARCHAR PRIMARY KEY,
    owner          VARCHAR NOT NULL,
    name           VARCHAR NOT NULL,
    description    VARCHAR DEFAULT NULL,
    cover_media_id VARCHAR DEFAULT NULL,
    is_archived    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(owner) REFERENCES accounts(account_id),
    FOREIGN KEY(cover_media_id) REFERENCES media(uuid)
);

-- Many-to-many: media items belonging to an album
CREATE TABLE IF NOT EXISTS album_media (
    album_id  VARCHAR NOT NULL,
    media_id  VARCHAR NOT NULL,
    position  INTEGER NOT NULL DEFAULT 0,
    added_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (album_id, media_id),
    FOREIGN KEY(album_id) REFERENCES albums(album_id),
    FOREIGN KEY(media_id) REFERENCES media(uuid)
);

-- Albums shared with a customer
CREATE TABLE IF NOT EXISTS customer_albums (
    customer_id VARCHAR NOT NULL,
    album_id    VARCHAR NOT NULL,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (customer_id, album_id),
    FOREIGN KEY(customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY(album_id) REFERENCES albums(album_id)
);

-- Per-customer manual item selection within an album.
-- When no rows exist for (customer_id, album_id) the customer sees all items.
-- When rows are present the customer only sees those specific items.
CREATE TABLE IF NOT EXISTS customer_album_items (
    customer_id VARCHAR NOT NULL,
    album_id    VARCHAR NOT NULL,
    media_id    VARCHAR NOT NULL,
    PRIMARY KEY (customer_id, album_id, media_id),
    FOREIGN KEY(customer_id) REFERENCES customers(customer_id),
    FOREIGN KEY(album_id) REFERENCES albums(album_id),
    FOREIGN KEY(media_id) REFERENCES media(uuid)
);

CREATE INDEX IF NOT EXISTS idx_albums_owner ON albums(owner);
CREATE INDEX IF NOT EXISTS idx_album_media_album ON album_media(album_id);
CREATE INDEX IF NOT EXISTS idx_customer_albums_customer ON customer_albums(customer_id);
