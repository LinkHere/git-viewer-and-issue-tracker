use super::browse::{fetch_repo_subtree, read_blob, show_repo_root};
use super::helper;
use super::model::{NewRepo, SubtreeParams, SubtreeParams as ReadBlobParams};
use super::service::{check_repo_id, fetch_all_repos, insert_repo, is_repo_exists};
use super::template::{
    render_all_repos_mrkp, render_new_repo_mrkp, render_repo_blob_mrkp, render_repo_files_mrkp,
    render_repo_subtree_mrkp,
};
use crate::common::helpers::validate_url_id;
use crate::common::html_layout::layout;
use crate::error::AppError;
use crate::states::AppState;
use axum::Form;
use axum::extract::rejection::PathRejection;
use axum::extract::{Path, State};
use axum::response::Redirect;
use maud::Markup;
use std;
use std::sync::Arc;

#[axum::debug_handler]
pub async fn read_repo_blob_handler(
    State(state): State<AppState>,
    params: Result<Path<ReadBlobParams>, PathRejection>,
) -> Result<Markup, AppError> {
    let Path(SubtreeParams {
        repo_id,
        repo_tree_path,
    }) = params?;
    let wrapped_id: Result<Path<i64>, PathRejection> = Ok(Path(repo_id));
    let valid_id = validate_url_id(wrapped_id);
    let repo_id = valid_id?;
    let repo_name_desc = check_repo_id(&state.pool, repo_id).await?;
    let repo_path = state.repo_path.join(&repo_name_desc.repo_name);
    let blobcontent = tokio::task::spawn_blocking(move || {
        let repo = state.repo(&repo_path)?;
        read_blob(&repo, &repo_tree_path)
    })
    .await?;
    println!("{:?}", blobcontent);
    let blobc = blobcontent?;
    Ok(layout(
        "Viewing Code",
        (String::from("Hello"), Some(String::from("World"))),
        render_repo_blob_mrkp(&blobc),
    ))
}

#[axum::debug_handler]
pub async fn list_repo_subtree_handler(
    State(state): State<AppState>,
    params: Result<Path<SubtreeParams>, PathRejection>,
) -> Result<Markup, AppError> {
    let Path(SubtreeParams {
        repo_id,
        repo_tree_path,
    }) = params?;
    let wrapped_id: Result<Path<i64>, PathRejection> = Ok(Path(repo_id));
    let valid_id = validate_url_id(wrapped_id);
    let repo_id = valid_id?;
    let repo_name_desc = check_repo_id(&state.pool, repo_id).await?;
    let repo_path = state.repo_path.join(&repo_name_desc.repo_name);
    let items = tokio::task::spawn_blocking(move || {
        let repo = state.repo(&repo_path)?;
        fetch_repo_subtree(&repo, &repo_tree_path)
    })
    .await?;
    println!("{:?}", items);
    Ok(layout(
        "Repo SubTree",
        (String::from("Hello"), Some(String::from("World"))),
        render_repo_subtree_mrkp(repo_id, &items?),
    ))
}

pub async fn list_repo_files_handler(
    State(state): State<AppState>,
    id: Result<Path<i64>, PathRejection>,
) -> Result<Markup, AppError> {
    let valid_id = validate_url_id(id)?;
    let repo_name_desc = check_repo_id(&state.pool, valid_id).await?;
    let repo_path = state.repo_path.join(&repo_name_desc.repo_name);
    let items = tokio::task::spawn_blocking(move || {
        let repo = state.repo(&repo_path)?;
        show_repo_root(&repo)
    })
    .await?;
    println!("{:?}", items);
    Ok(layout(
        "Repo Tree",
        (repo_name_desc.repo_name, repo_name_desc.repo_description),
        render_repo_files_mrkp(valid_id, items?),
    ))
}

//#[axum::debug_handler]
pub async fn list_all_repos_handler(State(state): State<AppState>) -> Result<Markup, AppError> {
    let repos = fetch_all_repos(&state.pool).await?;
    let repo_path = Arc::clone(&state.repo_path);

    let repo_with_dt: Vec<_> = tokio::task::spawn_blocking(move || {
        let mut path = repo_path.to_path_buf();
        path.reserve(96);

        repos
            .into_iter()
            .map(|repo| {
                path.push(&repo.repo_name);
                path.push("info/web/last-modified.txt");
                let dt = helper::read_repo_last_updated(&path);
                for _ in 0..4 {
                    path.pop();
                }
                (repo, dt)
            })
            .collect()
    })
    .await?;

    Ok(layout(
        "Repositories",
        (String::from("Hello"), Some(String::from("World"))),
        render_all_repos_mrkp(&repo_with_dt),
    ))
}

#[axum::debug_handler]
pub async fn new_repo_handler() -> Markup {
    layout(
        "Create New Repository",
        (String::from("Hello"), Some(String::from("World"))),
        render_new_repo_mrkp(),
    )
}

#[axum::debug_handler]
pub async fn create_repo_init_handler(
    State(state): State<AppState>,
    Form(payload): Form<NewRepo>,
) -> Result<Redirect, AppError> {
    let new_repo = NewRepo::normalized(&payload.repo_name, payload.repo_description.as_deref())?;

    if is_repo_exists(&state.pool, &new_repo.repo_name).await? {
        return Err(AppError::Conflict("Repo Already Exists!".into()));
    }

    let repo_path = state.repo_path.join(&new_repo.repo_name);
    let thread_block_path = repo_path.clone();

    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        match std::fs::create_dir(thread_block_path.clone()) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(AppError::Conflict("repo already exists on disk".into()));
            }
            Err(e) => return Err(e.into()),
        }

        helper::init_bare_repo(&thread_block_path).map_err(|e| {
            if let Err(e) = std::fs::remove_dir_all(&thread_block_path) {
                eprint!("{e}");
            }
            e
        })?;

        Ok(())
    })
    .await??;

    if let Err(e) = insert_repo(&state.pool, new_repo).await {
        if let Err(re) = tokio::fs::remove_dir_all(&repo_path).await {
            eprintln!("cleanup failed for {}: {re}", &repo_path.display());
        }
        return Err(e.into());
    }

    Ok(Redirect::to("/"))
}
