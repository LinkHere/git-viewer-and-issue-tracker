use super::browse::show_repo_root;
use super::helper;
use super::model::NewRepo;
use super::service::{check_repo_id, fetch_all_repos, insert_repo, is_repo_exists};
use super::template::{render_all_repos_mrkp, render_new_repo_mrkp};
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

pub async fn list_repo_files_handler(
    State(state): State<AppState>,
    id: Result<Path<i64>, PathRejection>
) -> Result<(), AppError> {
    let valid_id = validate_url_id(id)?;
    let repo_name = check_repo_id(&state.pool, valid_id).await?;
    let repo_path = state.repo_path.join(&repo_name);
    let items = tokio::task::spawn_blocking(move || {
        let repo = state.repo(&repo_path)?;
        show_repo_root(&repo)
    })
    .await?;
    println!("{:?}", items);
    Ok(())
}


//#[axum::debug_handler]
pub async fn list_all_repos_handler(
    State(state): State<AppState>
) -> Result<Markup, AppError> {

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
    }).await?;

    Ok(
        layout("Repositories", render_all_repos_mrkp(&repo_with_dt))
    )
}

#[axum::debug_handler]
pub async fn new_repo_handler() -> Markup {
    layout("Create New Repository", render_new_repo_mrkp())
}

#[axum::debug_handler]
pub async fn create_repo_init_handler(
    State(state): State<AppState>,
    Form(payload): Form<NewRepo>
) -> Result<Redirect, AppError> {

	let new_repo = NewRepo::normalized(
		&payload.repo_name,
		payload.repo_description.as_deref()
	)?;

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
            if let Err(e) = std::fs::remove_dir_all(&thread_block_path){
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
