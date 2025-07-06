use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub username: String,
    pub exp: u64,
}
