use super::model::{IssueComments, IssueDetails};

pub async fn list(pool: &SqlitePool, valid_id: i64) -> Result<(Vec<IssueDetails>, Vec<IssueComments>), AppError> {
    let issue_details = sqlx::query_as!(
        IssueDetails,
        r#"
        SELECT
            title,
            body,
        FROM 
            issues
        WHERE
            issue_id = ?
        "#,
        valid_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let issue_comments = sqlx::query_as!(
        IssueComments,
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
    Ok((issue_details, issues))
}
