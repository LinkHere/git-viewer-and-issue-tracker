use super::model::{NewRepo, Repo};
use crate::error::AppError;
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool) -> Result<Vec<Repo>, AppError> {
    Ok(sqlx::query_as!(
        Repo,
        "SELECT id, name, cgit_url, description, created_at FROM repos"
    )
    .fetch_all(pool)
    .await?)
}

pub async fn insert(payload: NewRepo, pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO repos (name, cgit_url, description)
        VALUES (?, ?, ?)
        "#,
        payload.name,
        payload.cgit_url,
        payload.description
    )
    .execute(pool)
    .await?;
    Ok(())
}
