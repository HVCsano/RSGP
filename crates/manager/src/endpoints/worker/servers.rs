use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use rsgp_shared::structs::ChangeServerStateBody;

use crate::conf::{
    loader::{load_servers, write_servers},
    structs::{Server, WorkersExt},
};

#[debug_handler]
pub async fn worker_change_state(
    w: Extension<WorkersExt>,
    Json(b): Json<ChangeServerStateBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut servers = load_servers().await;
    let srv = servers.get(&b.server);
    if srv.is_none() {
        return Err((StatusCode::NOT_FOUND, "No server found".to_string()));
    }
    let srv = srv.unwrap();
    if srv.worker != w.name {
        return Err((StatusCode::FORBIDDEN, "Not the same worker".to_string()));
    }
    servers.insert(
        b.server,
        Server {
            state: b.newstate,
            ..srv.clone()
        },
    );
    write_servers(servers).await;
    Ok(())
}
