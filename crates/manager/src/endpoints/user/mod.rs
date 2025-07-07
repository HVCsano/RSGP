use axum::{
    Extension, Json, Router, debug_handler, middleware, response::IntoResponse, routing::get,
};

use crate::{config::structs::User, endpoints::auth};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(auth_home))
        .layer(middleware::from_fn(auth::auth_middle))
}

#[debug_handler]
pub async fn auth_home(ext: Extension<User>) -> impl IntoResponse {
    Json(ext.0)
}
