use axum::{
    Extension, Json, Router, debug_handler, middleware,
    response::IntoResponse,
    routing::{get, post},
};

use crate::{conf::structs::UserExt, endpoints::auth};

mod admin;
mod servers;
mod session;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(auth_home))
        .route("/servers/get", get(servers::get_own_servers))
        .route("/sessions/changename", post(session::user_post_change_name))
        .route("/sessions/get", get(session::user_get_sessions))
        .route("/sessions/remove", post(session::user_post_remove_session))
        .route(
            "/sessions/remove_all",
            post(session::user_post_remove_all_session),
        )
        .nest("/admin", admin::routes())
        .layer(middleware::from_fn(auth::auth_middle))
}

#[debug_handler]
pub async fn auth_home(ext: Extension<UserExt>) -> impl IntoResponse {
    Json(ext.0)
}
