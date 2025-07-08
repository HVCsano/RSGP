use std::{collections::HashMap, path::Path};

use base64::{Engine, engine::general_purpose};
use rand::{TryRngCore, rngs::OsRng};
use tokio::fs::{self, File};
use tracing::{info, warn};

use crate::{
    config::structs::{
        GroupsConfig, Permissions, ServiceConfig, Session, SessionsConfig, User, UsersConfig,
    },
    utils::hash::hash_str,
};

pub async fn load_configs() {
    let path = Path::new("./config");
    if !path.exists() {
        fs::create_dir(path).await.unwrap();
    }
    let groups = Path::new("./config/groups.json");
    if !groups.exists() {
        warn!("No groups.json found, creating default group 'admin'");
        let mut groupsc: HashMap<String, Vec<Permissions>> = HashMap::new();
        groupsc.insert("admin".to_string(), vec![Permissions::Admin]);
        fs::write(groups, serde_json::to_string_pretty(&groupsc).unwrap())
            .await
            .unwrap();
        info!("Default admin group created.")
    }
    let users = Path::new("./config/users.json");
    if !users.exists() {
        warn!("No users.json found, creating default admin with password 'admin'");
        let user = UsersConfig {
            users: vec![User {
                username: "admin".to_string(),
                password: hash_str("admin"),
                groups: vec!["admin".to_string()],
            }],
        };
        fs::write(users, serde_json::to_string_pretty(&user).unwrap())
            .await
            .unwrap();
        info!("Default admin user created.");
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
    let sessions = Path::new("./config/sessions.json");
    if !sessions.exists() {
        warn!("No sessions.json found, creating empty one.");
        fs::write(
            sessions,
            serde_json::to_string_pretty(&SessionsConfig::new()).unwrap(),
        )
        .await
        .unwrap();
        info!("sessions.json created.")
    };
}

pub async fn load_users() -> UsersConfig {
    let users = File::open("./config/users.json").await.unwrap();
    serde_json::from_reader(users.into_std().await).expect("Users config is invalid format.")
}

pub async fn load_groups() -> GroupsConfig {
    let groups = File::open("./config/groups.json").await.unwrap();
    serde_json::from_reader(groups.into_std().await).expect("Groups config is invalid format.")
}

pub async fn get_groups_perm(groups: Vec<String>) -> Vec<Permissions> {
    let groupc = load_groups().await;
    let mut perms = Vec::new();
    for i in groups.iter() {
        let group = groupc.get(i);
        if group.is_some() {
            for j in group.cloned().unwrap().iter() {
                perms.push(j.clone());
            }
        }
    }
    perms
}

pub async fn load_service() -> ServiceConfig {
    let service = File::open("./config/service.json").await.unwrap();
    serde_json::from_reader(service.into_std().await).expect("Service config is invalid format.")
}

pub async fn load_sessions() -> SessionsConfig {
    let sessions = File::open("./config/sessions.json").await.unwrap();
    serde_json::from_reader(sessions.into_std().await)
        .expect("Sessions config is in invalid format.")
}

pub async fn add_session(id: String, name: String, agent: String, exp: i64) {
    let mut sessions = load_sessions().await;
    sessions.insert(
        id,
        Session {
            agent,
            username: name,
            login: chrono::Utc::now().timestamp(),
            exp,
        },
    );
    fs::write(
        "./config/sessions.json",
        serde_json::to_string_pretty(&sessions).unwrap(),
    )
    .await
    .unwrap();
}

pub async fn get_session(id: String) -> Option<Session> {
    let sessions = load_sessions().await;
    sessions.get(&id).cloned()
}
