use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    conf::{
        loader::{load_groups, load_users, write_groups, write_users},
        structs::{Permissions, PermissionsModifiers, User, UserExt},
    },
    utils::functions::atleast_one_permission,
};

#[debug_handler]
pub async fn admin_get_groups(
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
pub struct AddGroupBody {
    name: String,
}

#[debug_handler]
pub async fn admin_add_group(
    ext: Extension<UserExt>,
    Json(b): Json<AddGroupBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Groups(PermissionsModifiers::Write)],
        &ext.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut groups = load_groups().await;

    let our_group = groups.get(&b.name);
    if our_group.is_some() {
        return Err((StatusCode::BAD_REQUEST, "User already exists".to_string()));
    }

    groups.insert(b.name, Vec::new());
    write_groups(groups).await;
    return Ok(());
}

#[derive(Debug, Deserialize)]
pub struct GroupsRemoveBody {
    name: String,
}

#[debug_handler]
pub async fn admin_groups_remove(
    e: Extension<UserExt>,
    Json(b): Json<GroupsRemoveBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Groups(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut groups = load_groups().await;
    groups.retain(|k, _| k != &b.name);
    write_groups(groups).await;
    let mut users = load_users().await;
    for (k, v) in users.clone().iter() {
        if v.groups.contains(&b.name) {
            let mut groups = v.groups.clone();
            groups.remove(groups.iter().position(|p| p == &b.name).unwrap());
            let newuser = User {
                groups,
                password: v.password.clone(),
            };
            users.insert(k.clone(), newuser);
        }
    }
    write_users(users).await;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct AdminSetGroupPerms {
    group: String,
    perms: Vec<String>,
}

pub async fn admin_set_group_perms(
    e: Extension<UserExt>,
    Json(b): Json<AdminSetGroupPerms>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Groups(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to groups list".to_string(),
        ));
    }
    let mut groups = load_groups().await;
    let group = groups.get(&b.group);
    if group.is_none() {
        return Err((StatusCode::NOT_FOUND, "Group not found".to_string()));
    }
    let mut newperms = Vec::new();
    for perm in b.perms {
        let end = perm.split("/").collect::<Vec<&str>>();
        match perm.split("/").collect::<Vec<&str>>()[0] {
            "login" => newperms.push(Permissions::Login),

            "adminpage" => newperms.push(Permissions::AdminPage),

            "admin" => newperms.push(Permissions::Admin),

            "user" => match end[1] {
                "write" => {
                    newperms.push(Permissions::User(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::User(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "users" => match end[1] {
                "write" => {
                    newperms.push(Permissions::Users(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::Users(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "groups" => match end[1] {
                "write" => {
                    newperms.push(Permissions::Groups(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::Groups(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "servers" => match end[1] {
                "write" => {
                    newperms.push(Permissions::Servers(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::Servers(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "workers" => match end[1] {
                "write" => {
                    newperms.push(Permissions::Workers(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::Workers(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "eggs" => match end[1] {
                "write" => {
                    newperms.push(Permissions::Eggs(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::Eggs(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            "sitesettings" => match end[1] {
                "write" => {
                    newperms.push(Permissions::SiteSettings(PermissionsModifiers::Write));
                }
                "read" => {
                    newperms.push(Permissions::SiteSettings(PermissionsModifiers::Read));
                }
                _ => continue,
            },
            _ => continue,
        };
    }
    groups.insert(b.group.clone(), newperms);
    write_groups(groups).await;
    Ok(())
}
