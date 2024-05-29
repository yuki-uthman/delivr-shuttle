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
    let invoice = sqlx::query(
        "INSERT INTO invoice (created_by_name,
                                created_date,
                                customer_id,
                                customer_name,
                                date,
                                invoice_id,
                                invoice_number,
                                status,
                                total)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING invoice_id",
    )
    .bind(invoice.created_by_name)
    .bind(invoice.created_date)
    .bind(invoice.customer_id)
    .bind(invoice.customer_name)
    .bind(invoice.date)
    .bind(invoice.invoice_id)
    .bind(invoice.invoice_number)
    .bind(invoice.status)
    .bind(invoice.total)
    .execute(&state.pool)
    .await
    .map_err(Error::from)?;

    if invoice.rows_affected() != 1 {
        return Err(Error::from("Failed to insert invoice"));
    }

    Ok((StatusCode::OK, "Success!"))
}
