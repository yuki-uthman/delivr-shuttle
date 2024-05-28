use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Invoice {
    created_by_name: String,
    created_date: String,
    customer_id: String,
    customer_name: String,
    invoice_id: String,
    invoice_number: String,
    line_items: Vec<LineItem>,
    date: String,
    status: String,
    total: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LineItem {
    item_id: String,
    item_total: f64,
    name: String,
    purchase_rate: f64,
    quantity: i32,
    rate: f64,
}
