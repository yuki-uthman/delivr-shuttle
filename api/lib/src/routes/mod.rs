use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::zoho::invoice::Invoice;
use crate::{AppState, Error, Result};

pub async fn health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

pub async fn database(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let result: std::result::Result<i32, sqlx::Error> =
        sqlx::query_scalar("SELECT 1").fetch_one(&state.pool).await;

    match result {
        Ok(result) => Ok((StatusCode::OK, result.to_string())),
        Err(e) => Err(Error::from(e)),
    }
}

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
