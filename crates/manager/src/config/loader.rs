use std::path::Path;

use base64::{Engine, engine::general_purpose};
use rand::{TryRngCore, rngs::OsRng};
use tokio::fs::{self, File};
use tracing::{info, warn};

use crate::{
    config::structs::{Permissions, ServiceConfig, User, UsersConfig},
    utils::hash::hash_str,
};

pub async fn load_configs() {
    let path = Path::new("./config");
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
    let users = Path::new("./config/users.json");
    if !users.exists() {
        warn!("No users.json found, creating default admin with password 'admin'");
        let user = UsersConfig {
            users: vec![User {
                username: "admin".to_string(),
                password: hash_str("admin"),
                permissions: vec![Permissions::Admin],
            }],
        };
        fs::write(users, serde_json::to_string_pretty(&user).unwrap())
            .await
            .unwrap();
        info!("Default admin created.");
    }
    let service = Path::new("./config/service.json");
    if !service.exists() {
        warn!("No service.json found, generating JWT key.");
        let mut rng = OsRng;
        let mut random_bytes = vec![0u8; 256];
        rng.try_fill_bytes(&mut random_bytes).unwrap();
        let rstr = general_purpose::URL_SAFE_NO_PAD.encode(&random_bytes);
        let serviceconf = ServiceConfig {
            name: "Rust Game Panel".to_string(),
            jwt_key: rstr,
            workers: Vec::new(),
        };
        fs::write(service, serde_json::to_string_pretty(&serviceconf).unwrap())
            .await
            .unwrap();
        info!("Service config created.");
    }
}

pub async fn load_users() -> UsersConfig {
    let users = File::open("./config/users.json").await.unwrap();
    serde_json::from_reader(users.into_std().await).expect("Users config is invalid format.")
}

pub async fn load_service() -> ServiceConfig {
    let service = File::open("./config/service.json").await.unwrap();
    serde_json::from_reader(service.into_std().await).expect("Service config is invalid format.")
}
