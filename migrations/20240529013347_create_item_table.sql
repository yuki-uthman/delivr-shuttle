-- Add migration script here

CREATE TABLE IF NOT EXISTS Item (
    item_id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    purchase_rate NUMERIC(10, 2) NOT NULL,
    rate NUMERIC(10, 2) NOT NULL
);
