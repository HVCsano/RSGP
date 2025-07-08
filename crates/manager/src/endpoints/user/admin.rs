use axum::{
    Extension, Json, Router, debug_handler,
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::StatusCode;

use crate::{
    config::{
        loader::{load_groups, load_sessions, load_users, write_sessions, write_users},
        structs::{Permissions, PermissionsModifiers, UserExt},
    },
    utils::functions::atleast_one_permission,
};

pub fn routes() -> Router {
    Router::new()
        .route("/users/get", get(admin_get_users))
        .route("/users/post", post(admin_post_users))
        .route("/groups/get", get(admin_get_groups))
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
    let users = load_users().await;
    Ok(Json(users))
}

#[debug_handler]
async fn admin_post_users(
    h: HeaderMap,
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
    let user = h.get("user");
    let modify = h.get("modify");
    if user.is_none() || modify.is_none() {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            "Modifiers are required (user, modify)".to_string(),
        ));
    }
    let user = user.unwrap().to_str().unwrap();
    let modify = modify.unwrap().to_str().unwrap();
    let mut users = load_users().await;
    for (k, _) in users.clone().iter() {
        if k.to_owned() != user.to_string() {
            continue;
        }
        if modify == "delete" {
            users.remove(k);
            let mut sessions = load_sessions().await;
            for (j, ses) in sessions.clone().iter() {
                if ses.username == k.to_owned() {
                    sessions.remove(j);
                }
            }
            write_sessions(sessions.clone()).await;
        }
    }
    write_users(users.clone()).await;
    Ok(())
}

#[debug_handler]
async fn admin_get_groups(
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Groups(PermissionsModifiers::Write),
            Permissions::Groups(PermissionsModifiers::Read),
        ],
        &ext.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let groups = load_groups().await;
    Ok(Json(groups))
}
