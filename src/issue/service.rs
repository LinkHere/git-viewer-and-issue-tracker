use super::model::{NewIssue, RepoIssues, IssueWithComments};
use crate::error::AppError;
use sqlx::SqlitePool;

pub async fn list_issue_with_comments(
    pool: &SqlitePool,
    valid_id: i64
) -> Result<Vec<IssueWithComments>, AppError> {
    let issue_with_comments = sqlx::query_as!(
		IssueWithComments,
		r#"
		SELECT
			i.title AS issue_title,
			i.body AS issue_body,
			i.status AS issue_status,
			i.created_at AS issue_created_at,
			i.updated_at AS issue_updated_at,
			c.body AS comment_body,
			c.created_at AS comment_created_at
		FROM
			issues i
		INNER JOIN
			comments c ON i.id = c.issue_id
		WHERE
			i.id = ?
		"#,
		valid_id
	)
	.fetch_all(pool)
    .await?;
	Ok(issue_with_comments)
}

pub async fn list(pool: &SqlitePool, valid_id: i64) -> Result<(String, Vec<RepoIssues>), AppError> {
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
