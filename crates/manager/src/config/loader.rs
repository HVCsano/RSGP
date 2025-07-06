use std::path::Path;

use base64::{Engine, engine::general_purpose};
use rand::{Rng, TryRngCore, distr::StandardUniform, rngs::OsRng};
use sha2::{Digest, Sha256};
use tokio::fs;
use tracing::{info, warn};

use crate::config::structs::{Permissions, ServiceConfig, User, UsersConfig};

pub async fn load_configs() {
    let path = Path::new("./config");
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
    let users = Path::new("./config/users.json");
    if !users.exists() {
        warn!("No users.json found, creating default admin with password 'admin'");
        let mut hasher = Sha256::new();
        hasher.update("admin");
        let user = UsersConfig {
            users: vec![User {
                username: "admin".to_string(),
                password: format!("{:x}", hasher.finalize()),
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
