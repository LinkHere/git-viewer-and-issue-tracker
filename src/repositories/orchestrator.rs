use crate::error::AppError;
use anyhow::Result;

pub fn fetch_branches(
    repo: &gix::Repository,
) -> Result<Vec<(String, gix::hash::ObjectId)>, AppError> {
    let reference = repo.references()?;
    let branches = reference.local_branches()?;
    let mut results = Vec::new();
    for branch in branches {
        let mut branch = branch?;
        let branch_name = branch.name().shorten().to_string();
        let branch_id = branch.peel_to_id()?.detach();
        results.push((branch_name, branch_id))
    }
    Ok(results)
}
