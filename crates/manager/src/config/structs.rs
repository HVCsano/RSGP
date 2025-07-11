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
    pub access: String,
    pub key: String,
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
