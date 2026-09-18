use super::model::{NewIssue, RepoIssues as ListIssue, RepoIssues};
use crate::error::AppError;
use crate::repo;
use sqlx::SqlitePool;

pub async fn get_issue_with_comments(
    pool: &SqlitePool,
    (repo_valid_id, issue_valid_id): (i64, i64)
) -> Result<(String, ListIssue), AppError> {
	let repo_query = sqlx::query_scalar!(
		"SELECT
			name
		FROM
			repos
		WHERE
			id = ?
		",
		repo_valid_id
	)
	.fetch_optional(pool);
	
    let issue_query = sqlx::query_as!(
		ListIssue,
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
			id = ?
		AND
			repo_id = ?
		"#,
		issue_valid_id,
		repo_valid_id
	)
	.fetch_optional(pool);
	
	let (repo_name, issue) = tokio::try_join!(repo_query, issue_query)?;
	let (repo_name, issue) = (repo_name.ok_or(AppError::NotFound)?, issue.ok_or(AppError::NotFound)?);
	
	Ok((repo_name, issue))
}

pub async fn get_repo_issues(pool: &SqlitePool, valid_id: i64) -> Result<(Vec<RepoIdName>, Vec<RepoIssues>), AppError> {
    let repo_query = repo::service::get_repo_id_name(pool, valid_id);

    let issues_query = sqlx::query_as!(
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
    .fetch_all(pool);
	
	let (repo_id_name, issues) = tokio::try_join!(repo_query, issues_query)?;
	let repo_id_name = repo_id_name.ok_or(AppError::NotFound)?;
	
    Ok((repo_id_name, issues))
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
