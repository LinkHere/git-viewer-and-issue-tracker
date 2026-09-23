use super::model::NewRepo;
use super::service::{fetch_all_repos, insert_repo, is_repo_exists};
use super::template::{render_all_repos_mrkp, render_new_repo_mrkp};
use crate::common::html_layout::layout;
use crate::error::AppError;
use crate::states::AppState;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use maud::Markup;
use std::path::PathBuf;
use std;

const STORAGE_DIR: &str = "./repositories";

#[axum::debug_handler]
pub async fn list_all_repos_handler(
	State(state): State<AppState>
) -> Result<Markup, AppError> {
	Ok(
		layout("Repositories", render_all_repos_mrkp(&fetch_all_repos(&state.pool).await?))
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

    let repo_pathbuf = PathBuf::from(STORAGE_DIR).join(&new_repo.repo_name);
    
    let repo_pathbuf = tokio::task::spawn_blocking(move || -> Result<PathBuf, AppError> {
        std::fs::create_dir_all(STORAGE_DIR)?;

        match std::fs::create_dir(&repo_pathbuf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(AppError::Conflict("repo already exists on disk".into()));
            }
            Err(e) => return Err(e.into()),
        }

        gix::init_bare(&repo_pathbuf).map_err(|e| {
            let _ = std::fs::remove_dir_all(&repo_pathbuf);
            e
        })?;


        Ok(repo_pathbuf)
    })
    .await??;

	let repo = NewRepo {
		repo_name: payload.repo_name,
		repo_description: payload.repo_description,
	};
    if let Err(e) = insert_repo(&state.pool, repo).await {
		if let Err(re) = tokio::fs::remove_dir_all(&repo_pathbuf).await {
			eprintln!("cleanup failed for {}: {re}", repo_pathbuf.display());
		}
		return Err(e.into());
    }
    
    Ok(Redirect::to("/"))
}
