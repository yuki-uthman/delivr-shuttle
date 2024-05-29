use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::zoho::invoice::Invoice;
use crate::{AppState, Error, Result};

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    invoice: Invoice,
}

pub async fn webhook(
    State(state): State<AppState>,
    Json(payload): Json<Payload>,
) -> Result<impl IntoResponse> {

    let invoice = payload.invoice;
    println!("{:#?}", invoice);

    // insert the invoice into the database
    sqlx::query(
        "INSERT INTO invoice (created_by_name,
                                created_date,
                                customer_id,
                                customer_name,
                                date,
                                invoice_id,
                                invoice_number,
                                status,
                                total)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(invoice.created_by_name)
    .bind(invoice.created_date)
    .bind(invoice.customer_id)
    .bind(invoice.customer_name)
    .bind(invoice.date)
    .bind(&invoice.invoice_id)
    .bind(invoice.invoice_number)
    .bind(invoice.status)
    .bind(invoice.total)
    .execute(&state.pool)
    .await
    .map_err(Error::from)?;

    let line_items = invoice.line_items;
    // check if line_item.item_id is already in Item table
    for line_item in line_items {
        let res = sqlx::query("SELECT item_id FROM item WHERE item_id = $1")
            .bind(&line_item.item_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(Error::from)?;

        if res.is_none() {
            // insert the item into the item table
            sqlx::query(
                "INSERT INTO item (item_id, name, purchase_rate, rate)
                    VALUES ($1, $2, $3, $4)",
            )
            .bind(&line_item.item_id)
            .bind(line_item.name)
            .bind(line_item.purchase_rate)
            .bind(line_item.rate)
            .execute(&state.pool)
            .await
            .map_err(Error::from)?;
        }

        // insert line_item into the line_item table
        sqlx::query(
            "INSERT INTO LineItem (line_item_id, invoice_id, item_id, item_total, quantity, rate)
                VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(&line_item.line_item_id)
        .bind(&invoice.invoice_id)
        .bind(line_item.item_id)
        .bind(line_item.item_total)
        .bind(line_item.quantity)
        .bind(line_item.rate)
        .execute(&state.pool)
        .await
        .map_err(Error::from)?;

    }

    Ok((StatusCode::OK, "Success!"))
}
