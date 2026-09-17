pub mod handler;
pub mod model;
pub mod service;
pub mod template;

use crate::states::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{id}", get(handler::list_repo_issues_handler))
        .route("/{id}/new", get(handler::new_repo_issues_handler))
        .route("/{id}/issues", post(handler::insert_repo_issues_handler))
}
