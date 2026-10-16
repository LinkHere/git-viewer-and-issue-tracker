use super::helper::gix_time_to_human;
use super::model::{RepoEntry, RepoSubtree};
use super::orchestrator::fetch_branches;
use crate::error::AppError;
use anyhow::Result;
use gix::revision::walk::Sorting;
use gix::traverse::commit::simple::CommitTimeOrder;

#[derive(Debug)]
pub enum RepoContent {
    NoCommits,
    Empty,
    Files(Vec<RepoEntry>),
}

#[derive(Debug)]
pub enum RepoResource {
    ATree(Vec<RepoSubtree>),
    AFile {
        name: String,
        content: String,
        size: u64,
    },
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

pub fn fetch_repo_subtree(repo: &gix::Repository, path: &str) -> Result<RepoResource, AppError> {
    let tree_path = path.trim_matches('/');
    if tree_path.is_empty() {
        return Err(AppError::NotFound);
    }

    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => return Err(AppError::NotFound),
    };
    let root_tree = commit.tree()?;

    let entry = root_tree
        .lookup_entry_by_path(std::path::Path::new(tree_path))?
        .ok_or(AppError::NotFound)?;

    let obj = repo.find_object(entry.oid())?;

    match obj.kind {
        gix::object::Kind::Tree => {
            let target_tree = obj.try_into_tree()?;
            let mut files = Vec::new();
            for entry in target_tree.iter() {
                let entry = entry?;
                let filename = entry.filename().to_string();

                let blob_size = if entry.mode().is_blob() {
                    let header = repo.find_header(entry.id())?;
                    header.size()
                } else {
                    0
                };

                files.push(RepoSubtree {
                    repo_name: filename,
                    repo_path: tree_path.to_string(),
                    blob_size: blob_size,
                    is_dir: entry.mode().is_tree(),
                });
            }
            Ok(RepoResource::ATree(files))
        }
        gix::objs::Kind::Blob => {
            let blob = obj.try_into_blob()?;
            let filename = entry.filename().to_string();
            let size = blob.data.len() as u64;
            Ok(RepoResource::AFile {
                name: filename,
                content: String::from_utf8_lossy(&blob.data).into_owned(),
                size,
            })
        }
        _ => return Err(AppError::NotFound),
    }
}

pub fn show_repo_root(repo: &gix::Repository) -> Result<RepoContent, AppError> {
    let commit = match repo.head_commit() {
        Ok(commit) => commit,
        Err(_) => return Ok(RepoContent::NoCommits),
    };
    let tree = commit.tree()?;
    let mut files = Vec::new();
    let mut head_oids = Vec::new();
    let mut last_commits: Vec<Option<gix::ObjectId>> = Vec::new();

    let branch = fetch_branches(repo)?;
    for (name, id) in branch {
        println!("Branch Name: {}, Hash: {:?}", name, id);
    }
    for entry in tree.iter() {
        let entry = entry?;
        let filename = entry.filename().to_string();

        let blob_size = if entry.mode().is_blob() {
            let header = repo.find_header(entry.id())?;
            header.size()
        } else {
            0
        };

        head_oids.push(entry.id().detach());
        last_commits.push(None);

        files.push(RepoEntry {
            entry_name: filename,
            author_name: String::new(),
            commit_message: String::new(),
            commit_hash: String::new(),
            last_modified: String::new(),
            blob_size,
            is_dir: entry.mode().is_tree(),
            is_blob: entry.mode().is_blob(),
        });
    }

    if files.is_empty() {
        return Ok(RepoContent::Empty);
    }

    let mut pending: Vec<usize> = (0..files.len()).collect();
    let mut walk = repo
        .rev_walk([commit.id().detach()])
        .sorting(Sorting::ByCommitTime(CommitTimeOrder::NewestFirst))
        .all()?;

    while let Some(item) = walk.next() {
        if pending.is_empty() {
            break;
        }

        let commit_id = item?;

        let hist_commit = repo.find_commit(commit_id.id)?;
        let hist_tree = hist_commit.tree()?;

        let mut parent_trees: Option<Vec<gix::Tree>> = None;
        let mut still_pending = Vec::new();

        for &i in &pending {
            let name = &files[i].entry_name;
            let cur = hist_tree
                .lookup_entry_by_path(name)?
                .map(|e| e.id().detach());

            if cur != Some(head_oids[i]) {
                still_pending.push(i);
                continue;
            }

            if parent_trees.is_none() {
                let mut trees = Vec::new();
                for pid in hist_commit.parent_ids() {
                    trees.push(repo.find_commit(pid.detach())?.tree()?);
                }
                parent_trees = Some(trees);
            }

            let mut same_as_a_parent = false;
            for parent_tree in parent_trees.as_ref().unwrap() {
                let prev = parent_tree
                    .lookup_entry_by_path(name)?
                    .map(|e| e.id().detach());
                if prev == cur {
                    same_as_a_parent = true;
                    break;
                }
            }

            if same_as_a_parent {
                still_pending.push(i);
            } else {
                last_commits[i] = Some(commit_id.id);
            }
        }
        pending = still_pending;
    }

    for i in 0..files.len() {
        if let Some(id) = last_commits[i] {
            let hist_commit = repo.find_commit(id)?;

            let author_ref = hist_commit.author()?;
            let name = String::from_utf8_lossy(&author_ref.name);
            let email = String::from_utf8_lossy(&author_ref.email);
            files[i].author_name = format!("{} <{}>", name, email);

            let message = hist_commit.message()?;
            let raw_msg = String::from_utf8_lossy(&message.summary()).into_owned();
            files[i].commit_message = if raw_msg.chars().count() > 50 {
                format!("{}...", raw_msg.chars().take(47).collect::<String>())
            } else {
                raw_msg
            };

            files[i].commit_hash = hist_commit.id().to_hex_with_len(7).to_string();

            files[i].last_modified = match author_ref.time() {
                Ok(t) => gix_time_to_human(t),
                Err(_) => "unknown".to_string(),
            };
        }
    }

    Ok(RepoContent::Files(files))
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
