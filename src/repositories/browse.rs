use super::model::{RepoEntry, RepoSubtree};
use crate::error::AppError;
use anyhow::Result;

#[derive(Debug)]
pub enum RepoContent {
    NoCommits,
    Empty,
    Files(Vec<RepoEntry>),
}

pub fn read_blob(repo: &gix::Repository, path: &str) -> Result<String, AppError> {
    let tree_path = path.trim_matches('/');
    if tree_path.is_empty() {
        return Err(AppError::NotFound);
    }

    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => return Err(AppError::NotFound),
    };
    let root_tree = commit.tree()?;

    let blob_contents = if let Some(entry) = root_tree.lookup_entry_by_path(tree_path)? {
        if entry.mode().is_blob() {
            let blob = repo.find_blob(entry.oid()).unwrap();
            String::from_utf8_lossy(&blob.data).into_owned()
        } else {
            return Err(AppError::NotFound);
        }
    } else {
        return Err(AppError::NotFound);
    };
    Ok(blob_contents)
}

pub fn fetch_repo_subtree(
    repo: &gix::Repository,
    path: &str,
) -> Result<Vec<RepoSubtree>, AppError> {
    let tree_path = path.trim_matches('/');
    if tree_path.is_empty() {
        return Err(AppError::NotFound);
    }

    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => return Err(AppError::NotFound),
    };
    let root_tree = commit.tree()?;

    let target_tree = if let Some(entry) = root_tree.lookup_entry_by_path(tree_path)? {
        if entry.mode().is_tree() {
            let obj = repo.find_object(entry.oid())?;
            obj.try_into_tree()?
        } else {
            return Err(AppError::NotFound);
        }
    } else {
        return Err(AppError::NotFound);
    };

    let mut files = Vec::new();
    for entry in target_tree.iter() {
        let entry = entry?;
        let filename = entry.filename().to_string();

        files.push(RepoSubtree {
            repo_name: filename,
            repo_path: tree_path.to_string(),
            is_dir: entry.mode().is_tree(),
        });
    }
    println!("BASE PATH: {}", path);
    Ok(files)
}

pub fn show_repo_root(repo: &gix::Repository) -> Result<RepoContent, AppError> {
    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => return Ok(RepoContent::NoCommits),
    };
    let tree = commit.tree()?;
    let mut files = Vec::new();

    for entry in tree.iter() {
        let entry = entry?;

        files.push(RepoEntry {
            entry_name: entry.filename().to_string(),
            is_dir: entry.mode().is_tree(),
            is_blob: entry.mode().is_blob(),
        })
    }

    if files.is_empty() {
        Ok(RepoContent::Empty)
    } else {
        Ok(RepoContent::Files(files))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_blob_handler() {
        let repo = gix::open("./repositories/my-super-linux").unwrap();
        let content = read_blob(&repo, "output.txt").unwrap();
        println!("Blob Contents: {content}");
        assert!(!content.is_empty());
    }
}
