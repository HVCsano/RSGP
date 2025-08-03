use std::{fs, path::Path};

use axum::{Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use rsgp_shared::structs::{ChangeServerStateBody, Egg, ServerStates};
use serde::Deserialize;
use tokio::{
    fs::{File, OpenOptions},
    io::AsyncReadExt,
    process::{self, Command},
};

use crate::{SERVERS, conf::loader::get_main_config};

pub async fn change_server_state(server: String, state: ServerStates) {
    let conf = get_main_config().await;
    let client = reqwest::Client::new();
    client
        .post(format!(
            "{}/worker/servers/changestate",
            conf.manager_url.unwrap()
        ))
        .bearer_auth(conf.key.unwrap())
        .json(&ChangeServerStateBody {
            server,
            newstate: state,
        })
        .send()
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
pub struct AddServerBody {
    name: String,
    egg: Egg,
}

#[debug_handler]
pub async fn a_add_server(
    Json(b): Json<AddServerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_main_config().await;
    process::Command::new("sh")
        .arg("-c")
        .arg(format!("mkdir {}/{}", conf.servers_folder, b.name))
        .output()
        .await
        .unwrap();
    change_server_state(b.name.clone(), ServerStates::Installing).await;
    tokio::spawn(async move {
        for cmd in b.egg.install.dependency_installs.iter() {
            for cmd2 in cmd.iter() {
                process::Command::new("sh")
                    .arg("-c")
                    .current_dir(format!("{}/{}", conf.servers_folder, b.name))
                    .arg(cmd2)
                    .output()
                    .await
                    .unwrap();
            }
        }
        for cmd in b.egg.install.egg_installs.iter() {
            for cmd2 in cmd.iter() {
                process::Command::new("sh")
                    .arg("-c")
                    .current_dir(format!("{}/{}", conf.servers_folder, b.name))
                    .arg(cmd2)
                    .output()
                    .await
                    .unwrap();
            }
        }
        change_server_state(b.name, ServerStates::Stopped).await;
    });
    Ok(())
}

#[debug_handler]
pub async fn a_run_server(
    Json(b): Json<AddServerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    change_server_state(b.name.clone(), ServerStates::Running).await;
    let conf = get_main_config().await;
    let mut srv = SERVERS.lock().await;
    let our_srv = srv.get(&b.name);
    if our_srv.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Server is already running.".to_string(),
        ));
    }
    srv.insert(
        b.name.clone(),
        tokio::spawn(async move {
            let log_path = format!("./logs/{}", &b.name);
            let p = Path::new(&log_path);
            if !p.exists() {
                fs::create_dir(p).unwrap();
            }
            let mut log = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(format!("{}/server.log", &log_path))
                .await
                .unwrap();
            let mut srv = Command::new("sh")
                .arg("-c")
                .current_dir(format!("{}/{}", conf.servers_folder, &b.name))
                .arg(b.egg.running.start_command)
                .stdout(std::process::Stdio::piped())
                .stdin(std::process::Stdio::piped())
                .spawn()
                .unwrap();
            let mut stdout = srv.stdout.take().unwrap();
            tokio::spawn(async move {
                tokio::io::copy(&mut stdout, &mut log).await.ok();
            });

            let _ = srv.wait().await;
        }),
    );
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct GetServerLogBody {
    pub name: String,
}

#[debug_handler]
pub async fn a_get_server_log(
    Json(b): Json<GetServerLogBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let p = format!("./logs/{}/server.log", b.name);
    let path = Path::new(&p);
    if !path.exists() {
        return Err((StatusCode::NOT_FOUND, "Log not found".to_string()));
    }
    let mut file = File::open(&path).await.unwrap();
    let mut dst = String::new();
    file.read_to_string(&mut dst).await.unwrap();
    Ok(dst.into_response())
}
