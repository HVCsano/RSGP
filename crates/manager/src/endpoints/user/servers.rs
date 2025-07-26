use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;

use crate::conf::{loader::load_servers, structs::UserExt};

#[debug_handler]
pub async fn get_own_servers(
    e: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut servers = load_servers().await;
    servers.retain(|_, v| v.owner == e.username);
    Ok(Json(servers))
}
