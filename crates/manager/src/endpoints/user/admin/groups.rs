use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    config::{
        loader::load_groups,
        structs::{Permissions, PermissionsModifiers, UserExt},
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
