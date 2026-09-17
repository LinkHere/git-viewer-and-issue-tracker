use anyhow::Result;
use axum::Router;
use tower_http::services::ServeDir;

mod common;
mod db;
mod error;
mod issue;
mod repo;
mod states;

use db::init_db;

use error::fallback;
use states::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = init_db().await?;
    let state = AppState { pool };
    let static_service = ServeDir::new("static");
    let app = Router::new()
        .nest("/repos", repo::routes())
        .nest("/repo", issue::routes())
        .nest_service("/static", static_service)
        .fallback(fallback)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("{:?}", listener);
    axum::serve(listener, app).await?;

    Ok(())
}
