use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub key: Option<String>,
    pub servers_folder: String,
    pub manager_url: Option<String>,
}
