use axum::{Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use rsgp_shared::structs::Egg;
use serde::Deserialize;
use tokio::process;

use crate::conf::loader::get_main_config;

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
    Ok(())
}
