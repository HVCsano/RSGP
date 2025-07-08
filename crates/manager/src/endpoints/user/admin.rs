use axum::{Extension, Json, Router, debug_handler, response::IntoResponse, routing::get};
use reqwest::StatusCode;

use crate::{
    config::{
        loader::load_users,
        structs::{Permissions, PermissionsModifiers, UserExt},
    },
    utils::functions::atleast_one_permission,
};

pub fn routes() -> Router {
    Router::new().route("/users", get(admin_get_users))
}

#[debug_handler]
async fn admin_get_users(
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Users(PermissionsModifiers::Write),
            Permissions::Users(PermissionsModifiers::Read),
        ],
        &ext.permissions,
    ) {
        return Err((StatusCode::FORBIDDEN, "No access to users list".to_string()));
    }
    let users = load_users().await.users;
    Ok(Json(users))
}
