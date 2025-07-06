pub enum PermissionsModifiers {
    Read,
    Write,
}

pub enum Permissions {
    Login,
    AdminPage,
    Users(PermissionsModifiers),
    Groups(PermissionsModifiers),
    SiteSettings(PermissionsModifiers),
}

pub struct Users {
    pub username: String,
    pub password: String,
    pub permissions: Vec<Permissions>,
}

pub struct ServiceConfig {
    pub name: String,
    pub workers: Vec<Workers>,
}

pub struct Workers {
    pub name: String,
    pub access: String,
    pub key: String,
}
