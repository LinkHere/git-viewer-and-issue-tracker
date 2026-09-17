use super::model::NewIssue;
use super::service::{insert, list, list_issue_with_comments, check_repo_exists};
use super::template::{list_repo_issues_mrkp, list_issue_with_comments_mrkp, new_repo_issues_mrkp};
use crate::common::{helpers::validate_url_id, html_layout::layout};
use crate::error::AppError;
use crate::states::AppState;
use anyhow::Result;
use axum::{
    Form,
    extract::{Path, State, rejection::PathRejection},
    response::Redirect,
};
use maud::Markup;

pub async fn list_issue_with_comments_handler(
	State(state): State<AppState>,
	id: Result<Path<i64>, PathRejection>
) -> Result<Markup, AppError> {
	let valid_id = validate_url_id(id)?;
	let issue_with_comments = list_issue_with_comments(&state.pool, valid_id).await?;
	Ok(layout(
		"Issue - Comments",
		list_issue_with_comments_mrkp(&issue_with_comments)
	))
}

pub async fn list_repo_issues_handler(
    id: Result<Path<i64>, PathRejection>,
    State(state): State<AppState>,
) -> Result<Markup, AppError> {
    let valid_id = validate_url_id(id)?;
    let (repo_name, issues) = list(&state.pool, valid_id).await?;
    Ok(layout(
        "Repository Issues",
        list_repo_issues_mrkp(&repo_name, &issues),
    ))
}

pub async fn new_repo_issues_handler(
    id: Result<Path<i64>, PathRejection>,
    State(state): State<AppState>
) -> Result<Markup, AppError> {
    let valid_id = check_repo_exists(validate_url_id(id)?, &state.pool).await?;
    Ok(layout("New Repo Issue", new_repo_issues_mrkp(valid_id)))
}

pub async fn insert_repo_issues_handler(
    id: Result<Path<i64>, PathRejection>,
    State(state): State<AppState>,
    Form(payload): Form<NewIssue>,
) -> Result<Redirect, AppError> {
    let valid_id = check_repo_exists(validate_url_id(id)?, &state.pool).await?;
    insert(valid_id, &state.pool, payload).await?;
    Ok(Redirect::to(&format!("/repo/{}", valid_id)))
}
