use anyhow::Result;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use crate::error::AppError;

fn write_agefile(repo_pathbuf: &PathBuf) -> Result<(), AppError> {
    let web_dir = repo_pathbuf.join("info").join("web");
    fs::create_dir_all(&web_dir)?;
    let time_now = chrono::Utc::now().to_rfc3339();
    fs::write(web_dir.join("last-modified.txt"), time_now)?;
    Ok(())
}

fn write_post_receive_hook(repo_pathbuf: &PathBuf) -> Result<(), AppError> {
    let hook_path = repo_pathbuf.join("hooks").join("post-receive");
    let hook_script = "#!/bin/sh\n\
        mkdir -p info/web\n\
        date --rfc-3339=seconds > info/web/last-modified.txt\n";
    fs::write(&hook_path, hook_script)?;
    let mut perms = std::fs::metadata(&hook_path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&hook_path, perms)?;
    Ok(())
}

pub fn init_bare_repo(repo_pathbuf: &PathBuf) -> Result<(), AppError> {
    gix::init_bare(repo_pathbuf)?;
    write_agefile(repo_pathbuf)?;
    write_post_receive_hook(repo_pathbuf)?;
    Ok(())
}
