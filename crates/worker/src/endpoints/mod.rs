use axum::{
    Router, debug_middleware,
    extract::Request,
    http::HeaderMap,
    middleware::{Next, from_fn},
    response::IntoResponse,
    routing::get,
};
use reqwest::StatusCode;

use crate::conf::loader::get_main_config;

mod check;
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
    let conf = get_main_config().await;
    if conf.key.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Worker needs to be setted up!".to_string(),
        ));
    }
    if auth.to_owned() != conf.key.unwrap() {
        return Err((StatusCode::FORBIDDEN, "403".to_string()));
    }
    Ok(n.run(r).await)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(check::a_check))
        .layer(from_fn(auth_middle))
}
