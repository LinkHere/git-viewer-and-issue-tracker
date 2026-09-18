use super::model::{NewIssue, RepoIssues as ListIssue};
use crate::error::AppError;
use sqlx::SqlitePool;

pub async fn get_issue_with_comments(
    pool: &SqlitePool,
    valid_id: i64
) -> Result<Vec<ListIssue>, AppError> {
    let issue  = sqlx::query_as!(
		ListIssue,
		r#"
		SELECT
			title
			body
			status
			created_at
			updated_at
		FROM
			issues
		WHERE
			id = ?
		"#,
		valid_id
	)
	.fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

	Ok(issue)
}

pub async fn get_repo_issues(pool: &SqlitePool, valid_id: i64) -> Result<(String, Vec<RepoIssues>), AppError> {
    let repo_name = sqlx::query_scalar!("SELECT name FROM repos WHERE id = ?", valid_id)
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)?;

    let issues = sqlx::query_as!(
        RepoIssues,
        r#"
        SELECT
            title,
            body,
            status,
            created_at,
            updated_at
        FROM
            issues
        WHERE
            repo_id = ?
        "#,
        valid_id
    )
    .fetch_all(pool)
    .await?;
    Ok((repo_name, issues))
}

pub async fn insert(repo_id: i64, pool: &SqlitePool, payload: NewIssue) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO issues (repo_id, title, body)
        VALUES (?, ?, ?)
        "#,
        repo_id,
        payload.title,
        payload.body
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn check_repo_exists(valid_id: i64, pool: &SqlitePool)
    -> Result<i64, AppError> {
    Ok(sqlx::query_scalar!(
        r#"
        SELECT id FROM repos
        WHERE id = ?
        "#,
        valid_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?)
}
