use crate::conf::structs::{Permissions, Protocol};

pub fn atleast_one_permission(p: Vec<Permissions>, u: &Vec<Permissions>) -> bool {
    if p.is_empty() || u.is_empty() {
        return false;
    }
    if u.contains(&Permissions::Admin) {
        return true;
    }

    for permission in p {
        if u.contains(&permission) {
            return true;
        }
    }
    false
}

pub fn get_protocol(p: Protocol) -> String {
    match p {
        Protocol::Http => "http".to_string(),
        Protocol::Https => "https".to_string(),
    }
}
