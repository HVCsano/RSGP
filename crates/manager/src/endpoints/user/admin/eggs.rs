use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    conf::{
        loader::{load_eggs, write_eggs},
        structs::{Egg, Permissions, PermissionsModifiers, UserExt},
    },
    utils::functions::atleast_one_permission,
};

#[debug_handler]
pub async fn admin_get_eggs(
    e: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![
            Permissions::Eggs(PermissionsModifiers::Write),
            Permissions::Eggs(PermissionsModifiers::Read),
        ],
        &e.permissions,
    ) {
        return Err((StatusCode::FORBIDDEN, "No access to eggs list".to_string()));
    }
    let eggs = load_eggs().await;
    Ok(Json(eggs))
}

#[derive(Debug, Deserialize)]
pub struct AddEggBody {
    pub name: String,
    pub egg: Egg,
}

#[debug_handler]
pub async fn admin_add_egg(
    e: Extension<UserExt>,
    Json(b): Json<AddEggBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !atleast_one_permission(
        vec![Permissions::Eggs(PermissionsModifiers::Write)],
        &e.permissions,
    ) {
        return Err((StatusCode::FORBIDDEN, "No access to eggs list".to_string()));
    }

    let mut eggs = load_eggs().await;
    // Add the new worker to the service configuration
    eggs.insert(b.name, b.egg);
    write_eggs(eggs).await;
    // Save the updated service configuration (not implemented here)
    // save_service(updated_workers).await;

    Ok(())
}
