use serde::{Deserialize, Serialize};
use crate::Date;

#[derive(Deserialize, Serialize, Debug)]
pub struct Invoice {
    pub created_by_name: String,
    pub created_date: Date,
    pub customer_id: String,
    pub customer_name: String,
    pub invoice_id: String,
    pub invoice_number: String,
    pub line_items: Vec<LineItem>,
    pub date: Date,
    pub status: String,
    pub total: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LineItem {
    pub line_item_id: String,
    pub item_id: String,
    pub item_total: f64,
    pub name: String,
    pub purchase_rate: f64,
    pub quantity: i32,
    pub rate: f64,
}
