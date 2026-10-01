//use crate::error::AppError;
use anyhow::Result;
//use std::path::Path;

pub fn show_repo_root(repo: &gix::Repository) -> Result<Vec<(String, bool)>> {
    let tree = repo.head_commit()?.tree()?;
    let mut files = Vec::new();
    for entry in tree.iter() {
        let entry = entry?;
        files.push((entry.filename().to_string(), entry.mode().is_tree()));
    }
    Ok(files)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn list_repo_files() {
        let repo = gix::open("./repositories/my-super-linux").unwrap();
        let entries = show_repo_root(&repo).unwrap();
        for (name, is_dir) in &entries {
            println!("{} {}", if *is_dir {"dir "} else {"file"}, name);
        }
        assert!(!entries.is_empty());
    }
}
