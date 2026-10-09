use super::helper::gix_time_to_human;
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
        let filename = entry.filename().to_string();
        
        let blob_size = if entry.mode().is_blob() {
            let header = repo.find_header(entry.id())?;
            header.size()
        } else {
            0
        };

        let mut walk = repo.rev_walk([commit.id().detach()]).all()?;
        let mut last_oid = None;
        let mut last_time = None;
        let mut author_name = String::new();
        let mut commit_message = String::new();
        let mut commit_hash = String::new();

        while let Some(Ok(commit_id)) = walk.next() {
            let hist_commit = repo.find_object(commit_id.id)?.try_into_commit()?;
            let hist_tree = hist_commit.tree()?;

            match hist_tree.lookup_entry_by_path(&filename)? {
                Some(hist_entry) => {
                    let current_oid = hist_entry.id().detach();

                    if last_oid.is_none() {
                        last_oid = Some(current_oid);
                        last_time = Some(hist_commit.time()?);
                        //let author = hist_commit.author()?;
                        //author_name = String::from_utf8_lossy(&author.name).into_owned();
                        //let message = hist_commit.message()?;
                        //commit_message = String::from_utf8_lossy(&message.summary()).into_owned();
                        //commit_hash = hist_commit.id().to_hex_short().to_string();
                        // Generates a clean 7-character string like "4a5e62f"
                        let author_ref = hist_commit.author()?;
                        let name = String::from_utf8_lossy(&author_ref.name);
                        let email = String::from_utf8_lossy(&author_ref.email);
                        author_name = format!("{} <{}>", name, email);
                        let message = hist_commit.message()?;
                        let raw_msg = String::from_utf8_lossy(&message.summary()).into_owned();
                        commit_message = if raw_msg.chars().count() > 50 {
                            format!("{}...", raw_msg.chars().take(47).collect::<String>())
                        } else {
                            raw_msg
                        };
                        commit_hash = hist_commit.id().to_hex_with_len(7).to_string();
                    } else if Some(current_oid) != last_oid {
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }

        files.push(RepoEntry {
            entry_name: filename,
            author_name: author_name,
            commit_message: commit_message,
            commit_hash: commit_hash,
            last_modified: gix_time_to_human(last_time.unwrap_or_default()),
            blob_size: blob_size,
            is_dir: entry.mode().is_tree(),
            is_blob: entry.mode().is_blob(),
        });
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
