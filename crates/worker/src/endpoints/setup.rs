use axum::{Json, debug_handler, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::conf::{
    loader::{get_main_config, write_main_config},
    structs::Config,
};

#[derive(Debug, Deserialize)]
pub struct SetupWorkerBody {
    pub key: String,
    pub folder: String,
}

#[debug_handler]
pub async fn setup_worker(
    Json(b): Json<SetupWorkerBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_main_config().await;
    if conf.key.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Worker have already been setted up.".to_string(),
        ));
    }
    let new_conf = Config {
        key: Some(b.key),
        servers_folder: b.folder,
    };
    write_main_config(new_conf).await;
    Ok(())
}
