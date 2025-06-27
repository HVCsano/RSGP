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
