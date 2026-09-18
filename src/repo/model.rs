use serde::Deserialize;

#[derive(Debug)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub cgit_url: String,
    pub description: Option<String>,
    pub created_at: String
}

#[derive(Deserialize, Debug)]
pub struct NewRepo {
    pub name: String,
    pub cgit_url: String,
    pub description: Option<String>
}

#[derive(Debug)]
pub struct RepoIdName {
	pub id: i64,
	pub name: String
}