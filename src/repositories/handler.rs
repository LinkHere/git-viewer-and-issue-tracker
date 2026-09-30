use super::helper;
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
use std::path::{Path,PathBuf};
use std;

#[axum::debug_handler]
pub async fn list_all_repos_handler(
	State(state): State<AppState>
) -> Result<Markup, AppError> {
    
    let repos = fetch_all_repos(&state.pool).await?;
    let base_path = Path::new("./repositories");
    
    let fl = for repo in &repos {
        let f_path = &mut base_path.join(&repo.repo_name);
            f_path.push("info/web/last-modified.txt");
        let dt = helper::read_repo_last_updated(f_path);
        println!("Path: {:?}, Result: {:?}", f_path, dt);
    };

    println!("To be feed on markup: {:?}", fl);
    /*
    let dirs = tokio::task::spawn_blocking().await?;    
    */
	Ok(
		layout("Repositories", render_all_repos_mrkp(&repos, "Mon, Sep 28"))
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
        std::fs::create_dir_all(thread_block_path.clone())?;

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

	let repo = NewRepo {
		repo_name: payload.repo_name,
		repo_description: payload.repo_description,
	};
    if let Err(e) = insert_repo(&state.pool, repo).await {
		if let Err(re) = tokio::fs::remove_dir_all(&repo_path).await {
			eprintln!("cleanup failed for {}: {re}", &repo_path.display());
		}
		return Err(e.into());
    }
    
    Ok(Redirect::to("/"))
}
