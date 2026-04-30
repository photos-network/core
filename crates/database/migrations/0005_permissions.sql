ALTER TABLE accounts ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE IF NOT EXISTS album_accounts (
    account_id VARCHAR NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    album_id   VARCHAR NOT NULL REFERENCES albums(album_id) ON DELETE CASCADE,
    role       VARCHAR NOT NULL DEFAULT 'viewer', -- 'owner' or 'viewer'
    granted_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (account_id, album_id)
);

CREATE INDEX IF NOT EXISTS idx_album_accounts_album_id  ON album_accounts(album_id);
CREATE INDEX IF NOT EXISTS idx_album_accounts_account_id ON album_accounts(account_id);
