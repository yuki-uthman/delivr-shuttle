use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

mod error;
pub use error::{Error, Result};

mod zoho;
pub use zoho::invoice::{Invoice, LineItem};

mod routes;
pub use routes::{
    check::{database, health},
    webhook,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn build_router(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/health", get(health))
        .route("/database", get(database))
        .route("/post", post(webhook))
        .with_state(state)
}
