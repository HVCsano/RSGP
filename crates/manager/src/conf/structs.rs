use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PermissionsModifiers {
    Read,
    Write,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Permissions {
    Login,
    AdminPage,
    User(PermissionsModifiers),
    Users(PermissionsModifiers),
    Groups(PermissionsModifiers),
    Servers(PermissionsModifiers),
    Workers(PermissionsModifiers),
    Eggs(PermissionsModifiers),
    SiteSettings(PermissionsModifiers),
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub password: String,
    pub groups: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserExt {
    pub username: String,
    pub password: String,
    pub groups: Vec<String>,
    pub permissions: Vec<Permissions>,
}

pub type UsersConfig = HashMap<String, User>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub jwt_key: String,
    pub workers: Vec<Workers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workers {
    pub name: String,
    pub access: WorkerAccess,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Protocol {
    Http,
    Https,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerAccess {
    pub address: String,
    pub port: String,
    pub protocol: Protocol,
}

pub type SessionsConfig = HashMap<String, Session>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub username: String,
    pub agent: String,
    pub login: i64,
    pub exp: i64,
}

pub type GroupsConfig = HashMap<String, Vec<Permissions>>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub owner: String,
    pub worker: String,
    pub egg: String,
}

pub type ServersConfig = HashMap<String, Server>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Egg {
    pub install: EggInstallConf,
    pub running: EggRunConf,
    pub version: String,
    pub upstream: Option<String>,
}

pub type EggsConfig = HashMap<String, Egg>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggInstallConf {
    pub dependency_installs: Vec<Vec<String>>,
    pub egg_installs: Vec<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EggRunConf {
    pub start_command: String,
    pub running_text: String,
}
