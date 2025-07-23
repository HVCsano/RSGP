use axum::{debug_handler, response::IntoResponse};
use reqwest::StatusCode;

#[debug_handler]
pub async fn a_check() -> Result<impl IntoResponse, (StatusCode, String)> {
    return Ok(());
}
