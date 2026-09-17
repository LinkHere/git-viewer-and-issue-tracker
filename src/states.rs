use axum::extract::FromRef;
use sqlx::SqlitePool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: SqlitePool,
}
