use axum::{
    debug_handler, debug_middleware, extract::Request, http::HeaderMap, middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use reqwest::StatusCode;
use tracing::warn;

use crate::{
    conf::{
        loader::{add_session, get_groups_perm, get_session, load_service, load_users},
        structs::{Permissions, UserExt},
    },
    jwt::structs::JWT,
    utils::{functions::atleast_one_permission, hash::hash_str},
};

#[debug_middleware]
pub async fn auth_middle(
    h: HeaderMap,
    mut r: Request,
    n: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = h.get("Authorization");
    if auth.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Missing auth".to_string()));
    }
    let auth = auth.unwrap().to_str().unwrap().to_string();
    if !auth.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth".to_string()));
    }
    let jwt = auth.split("Bearer ").collect::<Vec<&str>>()[1];
    let service = load_service().await;
    let decode = jsonwebtoken::decode(
        jwt,
        &DecodingKey::from_secret(&service.jwt_key.as_bytes()),
        &Validation::default(),
    );
    if decode.is_err() {
        warn!("Error: {}", decode.unwrap_err());
        return Err((StatusCode::UNAUTHORIZED, "Invalid JWT".to_string()));
    }
    let jwt_data: JWT = decode.unwrap().claims;
    let users = load_users().await;
    let session = get_session(jwt_data.session_id.clone()).await;
    if session.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid session".to_string()));
    }
    let session = session.unwrap();
    let user = users.iter().find(|u| u.0.to_owned() == session.username);
    if user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "User not found".to_string()));
    }
    let user = user.unwrap();
    let perms = get_groups_perm(user.1.groups.clone()).await;
    if !atleast_one_permission(vec![Permissions::Login], &perms) {
        return Err((StatusCode::NOT_ACCEPTABLE, "User is disabled".to_string()));
    }
    r.extensions_mut().insert(UserExt {
        username: user.0.clone(),
        password: user.1.password.clone(),
        groups: user.1.groups.clone(),
        permissions: perms,
    });
    Ok(n.run(r).await)
}

#[debug_handler]
pub async fn login(h: HeaderMap) -> Result<impl IntoResponse, (StatusCode, String)> {
    let u = h.get("username");
    let p = h.get("password");
    let a = h.get("agent");
    if u.is_none() || p.is_none() || a.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Missing login credentials".to_string(),
        ));
    }
    let u = u.unwrap().to_str().unwrap();
    let p = p.unwrap().to_str().unwrap();
    let a = a.unwrap().to_str().unwrap();
    let users = load_users().await;
    let user = users
        .iter()
        .find(|us| us.0 == u && us.1.password == hash_str(p));
    if user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }
    let user = user.unwrap();
    let perms = get_groups_perm(user.1.clone().groups).await;
    if !atleast_one_permission(vec![Permissions::Login], &perms) {
        return Err((StatusCode::NOT_ACCEPTABLE, "User is disabled".to_string()));
    }
    let service = load_service().await;
    let sess_id = uuid::Uuid::new_v4().to_string();
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::weeks(51))
        .unwrap()
        .timestamp();
    add_session(sess_id.clone(), u.to_string(), a.to_string(), exp).await;
    let jwt = JWT {
        session_id: sess_id,
        exp,
    };
    let jwt_encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(&service.jwt_key.as_bytes()),
    )
    .unwrap();
    Ok(jwt_encoded)
}
