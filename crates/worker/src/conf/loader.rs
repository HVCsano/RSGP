use std::path::Path;

use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing::{info, warn};

use crate::conf::structs::Config;

pub async fn load_configs() {
    let path = Path::new("./config");
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
    let main = Path::new("./config/main.json");
    if !main.exists() {
        warn!("No main.json found, creating empty main config...");
        let mainconf: Config = Config {
            key: None,
            servers_folder: "/var/rsgpw".to_string(),
        };
        fs::write(main, serde_json::to_string_pretty(&mainconf).unwrap())
            .await
            .unwrap();
        info!("Empty config created, use manager to initialize the worker.")
    }
}

pub async fn get_main_config() -> Config {
    let conf = File::open("./config/main.json").await.unwrap();
    serde_json::from_reader(conf.into_std().await).unwrap()
}

pub async fn write_main_config(conf: Config) {
    let mut main = File::options()
        .write(true)
        .truncate(true)
        .open("./config/main.json")
        .await
        .unwrap();
    main.write_all(serde_json::to_string_pretty(&conf).unwrap().as_bytes())
        .await
        .unwrap();
}
