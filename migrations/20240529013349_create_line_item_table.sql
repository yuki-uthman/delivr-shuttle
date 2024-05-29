-- Add migration script here

CREATE TABLE IF NOT EXISTS LineItem (
    line_item_id VARCHAR(255) PRIMARY KEY,
    invoice_id VARCHAR(255) REFERENCES Invoice(invoice_id) ON DELETE CASCADE,
    item_id VARCHAR(255) REFERENCES Item(item_id) ON DELETE RESTRICT,
    item_total NUMERIC(10, 2) NOT NULL,
    quantity INT NOT NULL,
    rate NUMERIC(10, 2) NOT NULL
);
