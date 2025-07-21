use axum::{Extension, Json, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    conf::{
        loader::load_service,
        structs::{Permissions, PermissionsModifiers, UserExt},
    },
    utils::functions::atleast_one_permission,
};

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
