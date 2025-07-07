use axum::{
    debug_handler, extract::Request, http::HeaderMap, middleware::Next, response::IntoResponse,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use reqwest::StatusCode;

use crate::{
    config::{
        loader::{load_service, load_users},
        structs::User,
    },
    jwt::structs::JWT,
    utils::hash::hash_str,
};

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
        return Err((StatusCode::UNAUTHORIZED, "Invalid JWT".to_string()));
    }
    let jwt_data: JWT = decode.unwrap().claims;
    let users = load_users().await;
    let user = users
        .users
        .iter()
        .find(|u| u.username == jwt_data.username && u.password == jwt_data.password);
    if user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "User not found".to_string()));
    }
    r.extensions_mut().insert(User {
        username: user.unwrap().username.clone(),
        password: user.unwrap().password.clone(),
        permissions: user.unwrap().permissions.clone(),
    });
    Ok(n.run(r).await)
}

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
        password: hash_str(p),
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
