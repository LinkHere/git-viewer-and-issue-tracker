use crate::error::AppError;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::rejection::PathRejection;

pub fn validate_url_id(id: Result<Path<i64>, PathRejection>) -> Result<i64, AppError> {
    let Ok(Path(valid_id)) = id else {
        return Err(AppError::NotFound);
    };
    Ok(valid_id)
}
