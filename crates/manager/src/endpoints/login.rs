use axum::{debug_handler, http::HeaderMap, response::IntoResponse};
use jsonwebtoken::{EncodingKey, Header};
use reqwest::StatusCode;

use crate::{
    config::loader::{load_service, load_users},
    jwt::structs::JWT,
    utils::hash::hash_str,
};

#[debug_handler]
pub async fn login(h: HeaderMap) -> Result<impl IntoResponse, (StatusCode, String)> {
    let u = h.get("username");
    let p = h.get("password");
    if u.is_none() || p.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Missing login credentials".to_string(),
        ));
    }
    let u = u.unwrap().to_str().unwrap();
    let p = p.unwrap().to_str().unwrap();
    let users = load_users().await;
    let user = users
        .users
        .iter()
        .find(|us| us.username == u && us.password == hash_str(p));
    if user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }
    let service = load_service().await;
    let jwt = JWT {
        username: u.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(6)).timestamp(),
    };
    let jwt_encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(&service.jwt_key.as_bytes()),
    )
    .unwrap();
    Ok(jwt_encoded)
}
