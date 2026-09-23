use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewRepo {
    pub repo_name: String,
    pub repo_description: Option<String>,
}
