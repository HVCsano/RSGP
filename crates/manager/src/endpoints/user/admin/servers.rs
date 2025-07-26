use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    conf::{
        loader::{load_eggs, load_servers, load_service, load_users, write_servers},
        structs::{Permissions, PermissionsModifiers, Server, UserExt},
    },
    utils::functions::atleast_one_permission,
};

#[debug_handler]
pub async fn admin_get_servers(
    e: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Servers(PermissionsModifiers::Write),
            Permissions::Servers(PermissionsModifiers::Read),
        ],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to servers list".to_string(),
        ));
    }
    let servers = load_servers().await;
    Ok(Json(servers))
}

#[derive(Debug, Deserialize)]
pub struct AddServerBody {
    pub name: String,
    pub owner: String,
    pub worker: String,
    pub egg: String,
}

#[debug_handler]
pub async fn admin_add_server(
    e: Extension<UserExt>,
    Json(b): Json<AddServerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Servers(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to servers list".to_string(),
        ));
    }
    let eggs = load_eggs().await;
    if eggs.iter().find(|(k, _)| k == &&b.egg).is_none() {
        return Err((StatusCode::BAD_REQUEST, "No egg found.".to_string()));
    }
    let users = load_users().await;
    if users.iter().find(|(k, _)| k == &&b.owner).is_none() {
        return Err((StatusCode::BAD_REQUEST, "No owner found.".to_string()));
    }
    let workers = load_service().await.workers;
    if workers.iter().find(|w| w.name == b.worker).is_none() {
        return Err((StatusCode::BAD_REQUEST, "No worker found.".to_string()));
    }
    let mut servers = load_servers().await;
    let key = uuid::Uuid::new_v4().to_string();
    // Add the new worker to the service configuration
    servers.insert(
        key,
        Server {
            state: crate::conf::structs::ServerStates::Created,
            egg: b.egg,
            name: b.name,
            owner: b.owner,
            worker: b.worker,
        },
    );
    write_servers(servers).await;
    // Save the updated service configuration (not implemented here)
    // save_service(updated_workers).await;

    Ok(())
}
