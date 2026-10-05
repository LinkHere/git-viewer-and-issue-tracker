use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewRepo {
    pub repo_name: String,
    pub repo_description: Option<String>,
}

#[derive(Debug)]
pub struct FetchRepos {
    pub id: i64,
    pub repo_name: String,
    pub repo_description: Option<String>,
    pub created_at: String,
}


// Used by Markup
#[derive(Debug)]
pub struct RepoEntry {
    pub entry_name: String,
    pub is_dir: bool,
}

// Used for showing Repo SubTree
#[derive(Debug)]
pub struct RepoSubtree {
    pub repo_name: String,
    pub repo_path: String,
    pub is_dir: bool,
}
