pub mod browse;
pub mod handler;
pub mod helper;
pub mod model;
pub mod normalize;
pub mod orchestrator;
pub mod service;
pub mod template;

use crate::states::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_all_repos_handler))
        .route("/repo/new", get(handler::new_repo_handler))
        .route("/{repo_id}", get(handler::list_repo_files_handler))
        .route(
            "/{repo_id}/tree/{*repo_tree_path}",
            get(handler::list_repo_subtree_handler),
        )
        .route(
            "/{repo_id}/blob/{*repo_tree_path}",
            get(handler::read_repo_blob_handler),
        )
        .route("/repo/new", post(handler::create_repo_init_handler))
}
