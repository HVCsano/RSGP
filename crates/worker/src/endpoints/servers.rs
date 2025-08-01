use axum::{Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use rsgp_shared::structs::{ChangeServerStateBody, Egg, ServerStates};
use serde::Deserialize;
use tokio::{
    fs::OpenOptions,
    process::{self, Command},
};

use crate::conf::loader::get_main_config;

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
    tokio::spawn(async move {
        let mut log = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open("./logs/server.log")
            .await
            .unwrap();
        let mut srv = Command::new("sh")
            .arg("-c")
            .current_dir(format!("{}/{}", conf.servers_folder, b.name))
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
    });
    Ok(())
}
