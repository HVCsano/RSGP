use axum::{Extension, Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use rsgp_shared::structs::{Egg, ServerStates};
use serde::{Deserialize, Serialize};

use crate::{
    conf::{
        loader::{load_eggs, load_servers, load_service},
        structs::UserExt,
    },
    utils::functions::get_protocol,
};

#[debug_handler]
pub async fn get_own_servers(
    e: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut servers = load_servers().await;
    servers.retain(|_, v| v.owner == e.username);
    Ok(Json(servers))
}

#[derive(Debug, Deserialize)]
pub struct GetServerInfoBody {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct PostServerInfoBody {
    pub name: String,
    pub id: String,
    pub egg: Egg,
    pub state: ServerStates,
}

#[debug_handler]
pub async fn get_server_info(
    e: Extension<UserExt>,
    Json(b): Json<GetServerInfoBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let servers = load_servers().await;
    let server = servers.get(&b.id);
    if server.is_none() {
        return Err((StatusCode::NOT_FOUND, "Server not found".to_string()));
    }
    let server = server.unwrap();
    if server.owner != e.username {
        return Err((StatusCode::NOT_FOUND, "Server not found".to_string()));
    }
    let eggs = load_eggs().await;
    let egg = eggs.get(&server.egg).unwrap();
    Ok(Json(PostServerInfoBody {
        name: server.name.clone(),
        egg: egg.clone(),
        id: b.id,
        state: server.state.clone(),
    }))
}

#[derive(Debug, Serialize)]
pub struct PostStartServerBody {
    name: String,
    egg: Egg,
}

#[debug_handler]
pub async fn start_server(
    e: Extension<UserExt>,
    Json(b): Json<GetServerInfoBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let servers = load_servers().await;
    let server = servers.get(&b.id);
    if server.is_none() {
        return Err((StatusCode::NOT_FOUND, "Server not found".to_string()));
    }
    let server = server.unwrap();
    if server.owner != e.username {
        return Err((StatusCode::NOT_FOUND, "Server not found".to_string()));
    }
    let eggs = load_eggs().await;
    let egg = eggs.get(&server.egg).unwrap();
    let workers = load_service().await.workers;
    let work = workers.get(&server.worker).unwrap();
    let client = reqwest::Client::new();
    client
        .post(format!(
            "{}://{}:{}/a/servers/run",
            get_protocol(work.access.protocol.clone()),
            work.access.address,
            work.access.port
        ))
        .bearer_auth(work.key.clone())
        .json(&PostStartServerBody {
            name: b.id,
            egg: egg.clone(),
        })
        .send()
        .await
        .unwrap();
    Ok(())
}
