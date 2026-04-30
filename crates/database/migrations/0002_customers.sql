CREATE TABLE IF NOT EXISTS customers (
    -- auto generated
    customer_id VARCHAR PRIMARY KEY,
    access_code VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMPTZ DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_customers_access_code ON customers(access_code);
