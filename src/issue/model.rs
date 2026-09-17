use serde::Deserialize;

#[derive(Debug)]
pub struct RepoIssues {
    pub title: String,
    pub body: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
pub struct NewIssue {
    pub title: String,
    pub body: String,
}
