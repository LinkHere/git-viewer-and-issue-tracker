use super::model::NewRepo;
use crate::error::AppError;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn insert_repo(
    pool: &SqlitePool,
    path: &str,
    payload: NewRepo
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO
            repositories
            (repo_name,
            repo_description,
            repo_path)
        VALUES
            (?, ?, ?)
        "#,
        payload.repo_name,
        payload.repo_description,
        path
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn is_repo_exists(
    pool: &SqlitePool,
    repo_name: &str
) -> Result<bool, AppError> {
    let exists: bool = sqlx::query_scalar!(
    "SELECT EXISTS(SELECT 1 FROM repositories WHERE repo_name = ?1)",
    name
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}
