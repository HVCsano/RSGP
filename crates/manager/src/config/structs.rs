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
    pub username: String,
    pub password: String,
    pub permissions: Vec<Permissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersConfig {
    pub users: Vec<User>,
}

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
