use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use serde_json::Value;
use sqlx::PgPool;

mod error;

pub use error::{Error, Result};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

async fn health() -> impl IntoResponse {
    StatusCode::OK.into_response()
}

async fn database(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let result: std::result::Result<i32, sqlx::Error> =
        sqlx::query_scalar("SELECT 1").fetch_one(&state.pool).await;

    match result {
        Ok(result) => Ok((StatusCode::OK, result.to_string())),
        Err(e) => Err(Error::from(e)),
    }
}

async fn handle_post(Json(payload): Json<Value>) -> Result<impl IntoResponse> {
    // Handle the received JSON payload
    println!();
    println!("{:#?}", payload);
    println!();

    Ok((StatusCode::OK, "Success!"))
}

pub fn build_router(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/health", get(health))
        .route("/database", get(database))
        .route("/post", post(handle_post))
        .with_state(state)
}
