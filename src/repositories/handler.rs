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
use std;

//#[axum::debug_handler]
pub async fn list_all_repos_handler(
    State(state): State<AppState>
) -> Result<Markup, AppError> {

    let repos = fetch_all_repos(&state.pool).await?;
    let repo_path = state.repo_path.clone();

    let repo_with_dt = tokio::task::spawn_blocking(move || {
        let mut records = Vec::with_capacity(repos.len());
        for repo in repos {
            let mut full_repo_path = repo_path.to_path_buf();
            
            full_repo_path.push(&repo.repo_name);
            full_repo_path.push("info/web/last-modified.txt");
            
            println!("Full Repo Path: {:?}", full_repo_path);
            
            let dt = helper::read_repo_last_updated(&full_repo_path);
            records.push((repo, dt));
        }
        println!("To be feed on render_all_repos_markup: {:?}", records);
        records
    }).await?;

    /*let repo_refs: Vec<(&FetchRepos, Option<DateTime<Utc>>)> = repo_with_dt
        .iter()
        .map(|(repo, dt)| (repo, *dt))
        .collect();
    */
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
