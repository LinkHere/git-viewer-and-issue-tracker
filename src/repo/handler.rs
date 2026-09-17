use super::model::NewRepo;
use super::service::{insert, list};
use super::template::*;
use crate::common::html_layout::layout;
use crate::error::AppError;
use crate::states::AppState;
use anyhow::Result;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use maud::Markup;

pub async fn list_repo_handler(State(state): State<AppState>) -> Result<Markup, AppError> {
    let repos = list(&state.pool).await?;
    Ok(layout("Repositories", list_repo_mrkp(&repos)))
}

pub async fn new_repo_handler() -> Markup {
    layout("New Repository", new_repo_mrkp())
}

pub async fn insert_repo_handler(
    State(state): State<AppState>,
    Form(payload): Form<NewRepo>,
) -> Result<Redirect, AppError> {
    insert(payload, &state.pool).await?;
    Ok(Redirect::to("/repos"))
}
