CREATE TABLE IF NOT EXISTS album_views (
    id          INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    album_id    VARCHAR NOT NULL REFERENCES albums(album_id) ON DELETE CASCADE,
    viewer_id   VARCHAR NOT NULL,
    viewer_role VARCHAR NOT NULL,
    viewed_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_album_views_album_id  ON album_views(album_id);
CREATE INDEX IF NOT EXISTS idx_album_views_viewer_id ON album_views(viewer_id);

CREATE TABLE IF NOT EXISTS media_downloads (
    id               INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    media_id         VARCHAR NOT NULL,
    album_id         VARCHAR,
    downloader_id    VARCHAR NOT NULL,
    downloader_role  VARCHAR NOT NULL,
    downloaded_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_media_downloads_media_id  ON media_downloads(media_id);
CREATE INDEX IF NOT EXISTS idx_media_downloads_album_id  ON media_downloads(album_id);
