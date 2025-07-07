use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub session_id: String,
    pub exp: i64,
}
