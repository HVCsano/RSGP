use axum::{
    Router,
    routing::{get, post},
};

mod eggs;
mod groups;
mod servers;
mod users;
mod workers;

pub fn routes() -> Router {
    Router::new()
        .route("/users/get", get(users::admin_get_users))
        .route(
            "/users/changepassword",
            post(users::admin_change_user_password),
        )
        .route("/users/changegroup", post(users::admin_change_user_group))
        .route("/users/new", post(users::admin_add_user))
        .route("/users/delete", post(users::admin_delete_user))
        .route("/users/getgroups", get(users::admin_get_user_groups))
        .route("/groups/get", get(groups::admin_get_groups))
        .route("/groups/add", post(groups::admin_add_group))
        .route("/groups/remove", post(groups::admin_groups_remove))
        .route("/groups/setperms", post(groups::admin_set_group_perms))
        .route("/workers/get", get(workers::admin_get_service_workers))
        .route("/workers/check", post(workers::admin_check_service_worker))
        .route("/workers/add", post(workers::admin_add_service_worker))
        .route(
            "/workers/delete",
            post(workers::admin_delete_service_worker),
        )
        .route("/eggs/get", get(eggs::admin_get_eggs))
        .route("/eggs/add", post(eggs::admin_add_egg))
        .route("/servers/get", get(servers::admin_get_servers))
        .route("/servers/add", post(servers::admin_add_server))
}
