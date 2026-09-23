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