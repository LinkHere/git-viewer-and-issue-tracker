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

#[derive(Debug)]
pub struct IssueWithComments {
    pub issue_title: String,
    pub issue_body: String,
    pub issue_status: String,
    pub issue_created_at: String,
    pub issue_updated_at: String,
    pub comment_body: String,
    pub comment_created_at: String,
}
