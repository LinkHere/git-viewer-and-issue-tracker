pub mod handler;
pub mod model;
pub mod normalize;
pub mod service;
pub mod template;

use crate::states::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::new_repo_handler))
        .route("/new", post(handler::create_repo_init_handler))
}
