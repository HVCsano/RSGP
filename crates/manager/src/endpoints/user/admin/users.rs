use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    conf::{
        loader::{load_groups, load_sessions, load_users, write_sessions, write_users},
        structs::{Permissions, PermissionsModifiers, User, UserExt},
    },
    utils::{functions::atleast_one_permission, hash::hash_str},
};

#[debug_handler]
pub async fn admin_get_users(
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

#[derive(Debug, Deserialize)]
pub struct DeleteUserBody {
    user: String,
}

#[debug_handler]
pub async fn admin_delete_user(
    ext: Extension<UserExt>,
    Json(b): Json<DeleteUserBody>,
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
    let mut users = load_users().await;
    let mut sessions = load_sessions().await;
    users.retain(|k, _| k != &b.user);
    sessions.retain(|_, v| v.username != b.user);
    write_sessions(sessions.clone()).await;

    write_users(users.clone()).await;
    Ok(())
}

#[debug_handler]
pub async fn admin_get_user_groups(
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Users(PermissionsModifiers::Write),
            Permissions::Users(PermissionsModifiers::Read),
        ],
        &ext.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let groups = load_groups().await;
    let grouplist: Vec<String> = groups.iter().map(|f| f.0.to_owned()).collect();
    Ok(Json(grouplist))
}

#[derive(Debug, Deserialize)]
pub struct ChangeUserPasswordBody {
    user: String,
    password: String,
    clearsessions: bool,
}

#[debug_handler]
pub async fn admin_change_user_password(
    ext: Extension<UserExt>,
    Json(b): Json<ChangeUserPasswordBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Users(PermissionsModifiers::Write)],
        &ext.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut users = load_users().await;
    let our_user = users.get(&b.user);
    if our_user.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            "No user matching the name found.".to_string(),
        ));
    }
    let mut our_user = our_user.unwrap().clone();
    if b.clearsessions {
        let mut sessions = load_sessions().await;
        for (k, v) in sessions.clone().iter() {
            if v.username == b.user {
                sessions.remove(k);
            }
        }
        write_sessions(sessions).await;
    };
    our_user.password = hash_str(&b.password);
    users.insert(b.user, our_user);
    write_users(users).await;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct AddUserBody {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn admin_add_user(
    ext: Extension<UserExt>,
    Json(b): Json<AddUserBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Users(PermissionsModifiers::Write)],
        &ext.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut users = load_users().await;

    let our_user = users.get(&b.username);
    if our_user.is_some() {
        return Err((StatusCode::BAD_REQUEST, "User already exists".to_string()));
    }

    users.insert(
        b.username,
        User {
            password: hash_str(&b.password),
            groups: Vec::new(),
        },
    );
    write_users(users).await;
    return Ok(());
}

#[derive(Debug, Deserialize)]
pub struct ChangeUserGroupBody {
    user: String,
    groups: Vec<String>,
}

pub async fn admin_change_user_group(
    e: Extension<UserExt>,
    Json(b): Json<ChangeUserGroupBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Users(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut users = load_users().await;
    let user = users.get(&b.user);
    if user.is_none() {
        return Err((StatusCode::NOT_FOUND, "No user found".to_string()));
    }
    let mut user = user.unwrap().clone();
    user.groups = b.groups;
    users.insert(b.user, user);
    write_users(users).await;
    Ok(())
}
