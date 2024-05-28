use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::zoho::invoice::Invoice;
use crate::Result;

pub mod check;

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    invoice: Invoice,
}

pub async fn handle_post(Json(invoice): Json<Payload>) -> Result<impl IntoResponse> {
    // Handle the received JSON payload
    println!();
    println!("{:#?}", invoice);
    println!();

    Ok((StatusCode::OK, "Success!"))
}
