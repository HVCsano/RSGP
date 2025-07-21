use axum::{
    Router,
    routing::{get, post},
};

mod groups;
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
}
