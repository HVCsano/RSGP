use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    conf::{
        loader::{load_service, write_service},
        structs::{Permissions, PermissionsModifiers, Protocol, UserExt},
    },
    utils::functions::{atleast_one_permission, get_protocol},
};

#[debug_handler]
pub async fn admin_get_service_workers(
    e: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Workers(PermissionsModifiers::Write),
            Permissions::Workers(PermissionsModifiers::Read),
        ],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to workers list".to_string(),
        ));
    }
    let service = load_service().await;
    Ok(Json(service.workers))
}

#[derive(Debug, Deserialize)]
pub struct CheckServiceWorkerBody {
    pub name: String,
}

#[debug_handler]
pub async fn admin_check_service_worker(
    e: Extension<UserExt>,
    Json(b): Json<CheckServiceWorkerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Workers(PermissionsModifiers::Read),
            Permissions::Workers(PermissionsModifiers::Write),
        ],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to workers list".to_string(),
        ));
    }
    let workers = load_service().await.workers;
    let our_worker = workers.get(&b.name);
    if our_worker.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            "No worker found matching the name.".to_string(),
        ));
    }
    let our_worker = our_worker.unwrap();
    let check = reqwest::get(format!(
        "{}://{}:{}/a",
        match our_worker.access.protocol {
            Protocol::Http => "http",
            Protocol::Https => "https",
        },
        our_worker.access.address,
        our_worker.access.port
    ))
    .await;
    if check.is_ok() {
        return Ok(());
    }
    return Err((StatusCode::BAD_REQUEST, "Failed".to_string()));
}

#[derive(Debug, Deserialize)]
pub struct AddWorkerBody {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub protocol: Protocol,
    pub folder: String,
}

#[derive(Debug, Serialize)]
pub struct SetupWorkerBody {
    pub folder: String,
    pub key: String,
    pub url: String,
}

#[debug_handler]
pub async fn admin_add_service_worker(
    e: Extension<UserExt>,
    Json(b): Json<AddWorkerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Workers(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to workers list".to_string(),
        ));
    }

    let service = load_service().await;
    let key = uuid::Uuid::new_v4().to_string();
    let new_worker = crate::conf::structs::Workers {
        access: crate::conf::structs::WorkerAccess {
            address: b.address.clone(),
            port: b.port.to_string(),
            protocol: b.protocol.clone(),
        },
        key: key.clone(),
    };
    // Add the new worker to the service configuration
    let mut updated_workers = service.workers;
    updated_workers.insert(b.name.clone(), new_worker);
    let client = reqwest::Client::new();
    let workersetup = client
        .post(format!(
            "{}://{}:{}/setup",
            get_protocol(b.protocol),
            b.address,
            b.port
        ))
        .json(&SetupWorkerBody {
            key,
            folder: b.folder,
            url: service.public_url.clone(),
        })
        .send()
        .await;
    if workersetup.is_err() {
        return Err((
            StatusCode::PAYMENT_REQUIRED,
            "Failed to setup worker".to_string(),
        ));
    }
    if !workersetup.unwrap().status().is_success() {
        return Err((
            StatusCode::PAYMENT_REQUIRED,
            "Failed to setup worker".to_string(),
        ));
    }
    write_service(crate::conf::structs::ServiceConfig {
        workers: updated_workers,
        ..service
    })
    .await;
    // Save the updated service configuration (not implemented here)
    // save_service(updated_workers).await;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct DeleteWorkerBody {
    pub worker: String,
}

#[debug_handler]
pub async fn admin_delete_service_worker(
    e: Extension<UserExt>,
    Json(b): Json<DeleteWorkerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Workers(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((
            StatusCode::FORBIDDEN,
            "No access to workers list".to_string(),
        ));
    }

    let mut service = load_service().await;
    service.workers.retain(|w, _| w != &b.worker);
    write_service(service).await;
    Ok(())
}
