use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

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

