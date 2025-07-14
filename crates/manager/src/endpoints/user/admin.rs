use axum::{
    Extension, Json, Router, debug_handler,
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    config::{
        loader::{load_groups, load_sessions, load_users, write_sessions, write_users},
        structs::{Permissions, PermissionsModifiers, User, UserExt},
    },
    utils::{functions::atleast_one_permission, hash::hash_str},
};

pub fn routes() -> Router {
    Router::new()
        .route("/users/get", get(admin_get_users))
        .route("/users/changepassword", post(admin_change_user_password))
        .route("/users/new", post(admin_add_user))
        .route("/users/post", post(admin_post_users))
        .route("/users/getgroups", get(admin_get_user_groups))
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
    let our_user = users.get(user);
    if our_user.clone().is_none() {
        return Err((StatusCode::NOT_FOUND, "No user found".to_string()));
    }
    let mut our_user = our_user.unwrap().clone();
    if modify == "delete" {
        users.remove(user);
        let mut sessions = load_sessions().await;
        for (j, ses) in sessions.clone().iter() {
            if ses.username == user {
                sessions.remove(j);
            }
        }
        write_sessions(sessions.clone()).await;
    }
    if modify == "addgroup" {
        let value = h.get("value");
        if value.is_none() {
            return Err((StatusCode::BAD_REQUEST, "No value set".to_string()));
        }
        let value = value.unwrap().to_str().unwrap();
        if !our_user.groups.contains(&value.to_string()) {
            our_user.groups.push(value.to_string());
            users.insert(
                user.to_string(),
                User {
                    ..our_user.to_owned()
                },
            );
        }
    }
    if modify == "removegroup" {
        let value = h.get("value");
        if value.is_none() {
            return Err((StatusCode::BAD_REQUEST, "No value set".to_string()));
        }
        let value = value.unwrap().to_str().unwrap();
        if our_user.groups.contains(&value.to_string()) {
            our_user
                .groups
                .remove(our_user.groups.iter().position(|p| p == value).unwrap());
            users.insert(
                user.to_owned(),
                User {
                    ..our_user.to_owned()
                },
            );
        }
    }
    if modify == "setgroup" {
        let value = h.get("value");
        if value.is_none() {
            return Err((StatusCode::BAD_REQUEST, "No value set".to_string()));
        }
        let value = value.unwrap().to_str().unwrap();
        users.insert(
            user.to_owned(),
            User {
                groups: vec![value.to_string()],
                ..our_user.to_owned()
            },
        );
    }

    write_users(users.clone()).await;
    Ok(())
}

#[debug_handler]
async fn admin_get_user_groups(
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

#[derive(Debug, Deserialize)]
struct ChangeUserPasswordBody {
    user: String,
    password: String,
    clearsessions: bool,
}

#[debug_handler]
async fn admin_change_user_password(
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
struct AddUserBody {
    username: String,
    password: String,
}

#[debug_handler]
async fn admin_add_user(
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
