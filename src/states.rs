use crate::error::AppError;
use axum::extract::FromRef;
use dashmap::DashMap;
use sqlx::SqlitePool;
use std::path::{Path,PathBuf};
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: SqlitePool,
    pub repo_path: Arc<Path>,
    pub repo_cache: Arc<DashMap<PathBuf, gix::ThreadSafeRepository>>
}

impl AppState {
    pub fn new(pool: SqlitePool, repo_path: impl AsRef<Path>) -> Self {
        Self {
            pool,
            repo_path: Arc::from(repo_path.as_ref()),
            repo_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn repo(&self, path: &Path) -> Result<gix::Repository, AppError> {
        if let Some(r) = self.repo_cache.get(path) {
            return Ok(r.to_thread_local());
        }
        let ts = gix::ThreadSafeRepository::open(path)?;
        let local = ts.to_thread_local();
        self.repo_cache.insert(path.to_path_buf(), ts);
        Ok(local)
    }
}
