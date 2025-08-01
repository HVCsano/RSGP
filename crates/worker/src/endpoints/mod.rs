use axum::{
    Router, debug_middleware,
    extract::Request,
    http::HeaderMap,
    middleware::{Next, from_fn},
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::StatusCode;

use crate::conf::loader::get_main_config;

mod check;
mod servers;
pub mod setup;

#[debug_middleware]
pub async fn auth_middle(
    h: HeaderMap,
    r: Request,
    n: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = h.get("Authorization");
    if auth.is_none() {
        return Err((StatusCode::FORBIDDEN, "403".to_string()));
    }
    let auth = auth.unwrap().to_str().unwrap();
    if !auth.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth".to_string()));
    }
    let jwt = auth.split("Bearer ").collect::<Vec<&str>>()[1];
    let conf = get_main_config().await;
    if conf.key.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Worker needs to be setted up!".to_string(),
        ));
    }
    if jwt.to_owned() != conf.key.unwrap() {
        return Err((StatusCode::FORBIDDEN, "403".to_string()));
    }
    Ok(n.run(r).await)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(check::a_check))
        .route("/servers/add", post(servers::a_add_server))
        .route("/servers/run", post(servers::a_run_server))
        .layer(from_fn(auth_middle))
}
