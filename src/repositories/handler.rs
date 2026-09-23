use super::normalize;
use super::model::NewRepo;
use super::service::{insert_repo, is_repo_exists};
use super::template::render_new_repo_mrkp;
use crate::common::html_layout::layout;
use crate::error::AppError;
use crate::states::AppState;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use maud::Markup;
use std::path::PathBuf;
use std::path::Path;
use std;

const STORAGE_DIR: &str = "./repositories";

pub async fn new_repo_handler() -> Markup {
    layout("Create New Repository", render_new_repo_mrkp())
}

#[axum::debug_handler]
pub async fn create_repo_init_handler(
    State(_state): State<AppState>,
    Form(payload): Form<NewRepo>
) -> Result<Redirect, AppError> {

    if is_repo_exists(&state.pool, &payload.repo_name).await? {
        return Err(AppError::Conflict("Repo Already Exists!".into()));
    }    

    let repo_pathbuf = PathBuf::from(STORAGE_DIR).join(format!("{name}.git"));
    //let path_str = repo_path.to_string_lossy().to_string();
    
    let repo_path = tokio::task::spawn_blocking(move || -> Result<PathBuf, AppError> {
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

    /*if let Err(e) = insert_repo(&state.pool).await {

    }*/
    
    Ok(Redirect::to("/"))
}
