use super::model::NewRepo;
use anyhow::Result;
use crate::error::AppError;

impl NewRepo {
    pub fn normalized(repo_name: &str, repo_description: Option<&str>) -> Result<Self, AppError> {
        let repo_name = repo_name.trim().to_lowercase();
        let valid = !repo_name.is_empty()
            && repo_name.len() <= 64
            && !repo_name.starts_with('.')
            && repo_name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-'));
        if !valid {
            return Err(AppError::BadRequest("invalid repo name".into()));
        }

        let repo_description = normalize_description(repo_description)?;

        Ok(Self { repo_name, repo_description })
    }
}

fn normalize_description(desc: Option<&str>) -> Result<Option<String>, AppError> {
    let Some(desc) = desc else { return Ok(None) };
    let trimmed = desc.trim();

    if trimmed.is_empty() {
        return Ok(None);
    }

    if trimmed.chars().any(|c| c.is_control() && c != ' ') {
        return Err(AppError::BadRequest("control characters not allowed".into()));
    }

    if trimmed.chars().count() > 350 {
        return Err(AppError::BadRequest("description too long (max 350 chars)".into()));
    }

    Ok(Some(trimmed.split_whitespace().collect::<Vec<_>>().join(" ")))
}
