use super::model::{RepoEntry,RepoSubtree};
use crate::error::AppError;
use anyhow::Result;
use gix::traverse::tree::Recorder;

#[derive(Debug)]
pub enum RepoContent {
    NoCommits,
    Empty,
    Files(Vec<RepoEntry>),
}

pub fn show_repo_root(repo: &gix::Repository) -> Result<RepoContent, AppError> {
    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => {
            return Ok(RepoContent::NoCommits)
        }
    };
    let tree = commit.tree()?;
    let mut files = Vec::new();
    for entry in tree.iter() {
        let entry = entry?;
        files.push(RepoEntry{
            entry_name: entry.filename().to_string(),
            is_dir: entry.mode().is_tree(),
        });
    }
    if files.is_empty() {
        Ok(RepoContent::Empty)
    } else {
        Ok(RepoContent::Files(files))
    }
}

pub fn show_repo_subtree(
    repo: &gix::
    path: &str
) -> Result <> {}

pub fn show_repo_recursively(repo: &gix::Repository) -> Result<Recorder, AppError> {
    let tree = repo.head_commit().unwrap().tree().unwrap();
    let mut recorder = Recorder::default();
    tree.traverse().breadthfirst(&mut recorder).unwrap();
    Ok(recorder)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn list_repo_files_recursively() {
        let repo = gix::open("./repositories/my-super-linux").unwrap();
        let recorder = show_repo_recursively(&repo).unwrap();
        for entry in recorder.records {
            println!("{:?} {} {}", entry.mode.kind(), entry.oid, entry.filepath);
        }
    }

    /*#[test]
    fn list_repo_files() {
        let repo = gix::open("./repositories/my-super-linux").unwrap();
        let entries = show_repo_root(&repo).unwrap();
        for (name, is_dir) in &entries {
            println!("{} {}", if *is_dir {"dir "} else {"file"}, name);
        }
        assert!(!entries.is_empty());
    }*/
}
