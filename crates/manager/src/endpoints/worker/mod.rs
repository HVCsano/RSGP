use axum::{
    Router, debug_middleware,
    extract::Request,
    http::HeaderMap,
    middleware::{self, Next},
    response::IntoResponse,
    routing::post,
};
use reqwest::StatusCode;

use crate::conf::loader::load_service;

mod servers;

pub fn routes() -> Router {
    Router::new()
        .route("/servers/changestate", post(servers::worker_change_state))
        .layer(middleware::from_fn(worker_auth))
}

#[debug_middleware]
pub async fn worker_auth(
    h: HeaderMap,
    mut r: Request,
    n: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = h.get("Authorization");
    if auth.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Missing auth".to_string()));
    }
    let auth = auth.unwrap().to_str().unwrap().to_string();
    if !auth.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth".to_string()));
    }
    let key = auth.split("Bearer ").collect::<Vec<&str>>()[1];
    let service = load_service().await;
    let worker = service.workers.iter().find(|p| p.key == key.to_string());
    if worker.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "No worker found".to_string()));
    }
    r.extensions_mut().insert(worker.unwrap().clone());
    Ok(n.run(r).await)
}
