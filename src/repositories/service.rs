use super::model::{FetchRepos, NewRepo};
use crate::common::types;
use crate::error::AppError;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_repo_id_name(
    pool: &SqlitePool,
    valid_id: i64,
) -> Result<Option<types::RepoIdName>, sqlx::Error> {
    Ok(sqlx::query_as!(
        types::RepoIdName,
        "SELECT id, repo_name FROM repositories WHERE id = ?",
        valid_id
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn fetch_all_repos(pool: &SqlitePool) -> Result<Vec<FetchRepos>, AppError> {
    Ok(sqlx::query_as!(
        FetchRepos,
        "SELECT id, repo_name, repo_description, created_at FROM repositories"
    )
    .fetch_all(pool)
    .await?)
}

pub async fn insert_repo(
    pool: &SqlitePool,
    payload: NewRepo
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO
            repositories
            (repo_name,
            repo_description)
        VALUES
            (?, ?)
        "#,
        payload.repo_name,
        payload.repo_description,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn is_repo_exists(
    pool: &SqlitePool,
    repo_name: &str
) -> Result<bool, AppError> {
    let exists: i64 = sqlx::query_scalar!(
    "SELECT COUNT(*) FROM repositories WHERE repo_name = ?1",
    repo_name
    )
    .fetch_one(pool)
    .await?;

    Ok(exists > 0)
}

pub async fn check_repo_id(
    pool: &SqlitePool,
    id: i64
) -> Result<String, AppError> {
    let repo_name = sqlx::query_scalar!("SELECT repo_name FROM repositories WHERE id = ?", id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(repo_name)
}
