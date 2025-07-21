use crate::conf::structs::Permissions;

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
