-- Add migration script here

CREATE TABLE IF NOT EXISTS line_items (
    line_item_id VARCHAR(255) PRIMARY KEY,
    invoice_id VARCHAR(255) REFERENCES invoices(invoice_id) ON DELETE CASCADE,
    item_id VARCHAR(255) REFERENCES items(item_id) ON DELETE RESTRICT,
    item_total NUMERIC(10, 2) NOT NULL,
    quantity INT NOT NULL,
    rate NUMERIC(10, 2) NOT NULL
);
