use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use bigdecimal::ToPrimitive;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use sqlx::{types::BigDecimal, FromRow};

use crate::{AppState, Date, Result};

#[allow(dead_code)]
#[derive(Debug, FromRow, Deserialize)]
struct Invoice {
    pub created_by_name: String,
    pub customer_name: String,
    pub date: Date,
    pub status: String,
    pub invoice_id: String,
}

#[allow(dead_code)]
#[derive(Debug, FromRow, Serialize)]
struct LineItem {
    pub item_id: Option<String>,
    pub quantity: i32,

    #[sqlx(skip)]
    pub item: Option<Item>,
}

#[allow(dead_code)]
#[derive(Debug, FromRow, Deserialize)]
struct Item {
    pub name: String,
    pub purchase_rate: BigDecimal,
    pub rate: BigDecimal,
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Item", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("purchase_rate", &self.purchase_rate.to_f64())?;
        state.serialize_field("rate", &self.rate.to_f64())?;
        state.end()
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct Sale {
    pub date: Date,
    pub customer_name: String,
    pub salesperson_name: String,
    pub line_items: Vec<LineItem>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceQuery {
    date: Option<String>,
}

async fn get_invoices_by_date(pool: &sqlx::PgPool, date: Date) -> Result<Vec<Invoice>> {
    let invoices: Vec<Invoice> = sqlx::query_as(
        r#"
        SELECT created_by_name, customer_name, date, status, invoice_id
        FROM invoices
        WHERE date = $1
        "#,
    )
    .bind(date)
    .fetch_all(pool)
    .await?;

    Ok(invoices)
}

async fn get_line_items_from_invoice(
    pool: &sqlx::PgPool,
    invoice: &Invoice,
) -> Result<Vec<LineItem>> {
    let line_items: Vec<LineItem> = sqlx::query_as(
        r#"
        SELECT item_id, quantity
        FROM line_items
        WHERE invoice_id = $1
        "#,
    )
    .bind(&invoice.invoice_id)
    .fetch_all(pool)
    .await?;

    Ok(line_items)
}

async fn get_item_from_line_item(pool: &sqlx::PgPool, line_item: &LineItem) -> Result<Item> {
    let item_id = line_item.item_id.clone().unwrap();
    let item = sqlx::query_as(
        r#"
        SELECT name, purchase_rate, rate
        FROM items
        WHERE item_id = $1
        "#,
    )
    .bind(item_id)
    .fetch_one(pool)
    .await?;

    Ok(item)
}

async fn get_sales_by_date(pool: &sqlx::PgPool, date: Date) -> Result<Vec<Sale>> {
    let invoices = get_invoices_by_date(pool, date).await?;

    let mut results = Vec::new();

    for invoice in invoices.iter() {
        let mut line_items = get_line_items_from_invoice(pool, invoice).await?;

        for line_item in line_items.iter_mut() {
            let item = get_item_from_line_item(pool, line_item).await?;
            line_item.item = Some(item);
        }

        let sales = Sale {
            date: invoice.date,
            customer_name: invoice.customer_name.clone(),
            salesperson_name: invoice.created_by_name.clone(),
            line_items,
        };
        results.push(sales);
    }

    Ok(results)
}

// expect date query to be in the format YYYY-MM-DD /invoices?date=2022-01-01
pub async fn invoices(
    State(state): State<AppState>,
    Query(query): Query<InvoiceQuery>,
) -> Result<impl IntoResponse> {
    println!("{:#?}", query);

    if query.date.is_none() {
        return Ok((StatusCode::OK, "No date provided".to_string()));
    }

    let date = query.date.unwrap();
    let date = date.trim();

    let date = Date::parse_from_str(date, "%Y-%m-%d").unwrap();

    let results = get_sales_by_date(&state.pool, date).await?;
    println!("{:#?}", results);

    let json = serde_json::to_string(&results).unwrap();

    Ok((StatusCode::OK, json))
}
