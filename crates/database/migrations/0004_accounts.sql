-- Links one account to multiple customer access codes
CREATE TABLE IF NOT EXISTS account_access_codes (
    account_id  VARCHAR NOT NULL,
    customer_id VARCHAR NOT NULL,
    linked_at   TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (account_id, customer_id),
    FOREIGN KEY(account_id) REFERENCES accounts(account_id),
    FOREIGN KEY(customer_id) REFERENCES customers(customer_id)
);

CREATE INDEX IF NOT EXISTS idx_account_access_codes_account ON account_access_codes(account_id);
