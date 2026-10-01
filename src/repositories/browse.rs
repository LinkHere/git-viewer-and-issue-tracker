use crate::error::AppError;
use anyhow::Result;
use std::path::Path;

pub fn open_repo(path: &Path) -> Result<>
