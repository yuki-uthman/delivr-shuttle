-- Add migration script here

CREATE TABLE IF NOT EXISTS Invoice (
    created_by_name VARCHAR(255) NOT NULL,
    created_date DATE NOT NULL,
    customer_id VARCHAR(255) NOT NULL,
    customer_name VARCHAR(255) NOT NULL,
    invoice_id VARCHAR(255) PRIMARY KEY,
    invoice_number VARCHAR(255) UNIQUE NOT NULL,
    date DATE NOT NULL,
    status VARCHAR(50) NOT NULL,
    total NUMERIC(10, 2) NOT NULL
);
