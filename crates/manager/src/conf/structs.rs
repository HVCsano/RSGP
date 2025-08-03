use std::collections::HashMap;

use rsgp_shared::structs::{Egg, ServerStates};
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
    pub warnings: Vec<Warning>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Warning {
    pub title: String,
    pub description: String,
}

pub type UsersConfig = HashMap<String, User>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub public_url: String,
    pub jwt_key: String,
    pub workers: HashMap<String, Workers>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workers {
    pub access: WorkerAccess,
    pub key: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkersExt {
    pub name: String,
    pub access: WorkerAccess,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Protocol {
    Http,
    Https,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub name: String,
    pub worker: String,
    pub egg: String,
    pub state: ServerStates,
}

pub type ServersConfig = HashMap<String, Server>;

pub type EggsConfig = HashMap<String, Egg>;
