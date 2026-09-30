use axum::extract::FromRef;
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: SqlitePool,
    pub repo_path: Arc<Path>
}
