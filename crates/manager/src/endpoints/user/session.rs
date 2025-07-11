use axum::{Extension, Json, debug_handler, http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;
use urlencoding::decode;

use crate::config::{
    loader::{load_sessions, write_sessions},
    structs::{Session, UserExt},
};

#[derive(Debug, Serialize)]
pub struct UserSessionsList {
    pub id: String,
    pub agent: String,
    pub login_time: i64,
    pub exp_time: i64,
}

#[debug_handler]
pub async fn user_get_sessions(ext: Extension<UserExt>) -> impl IntoResponse {
    let sessions = load_sessions().await;
    let mut list = Vec::new();
    for (k, v) in sessions.iter() {
        if v.username != ext.username {
            continue;
        }
        list.push(UserSessionsList {
            agent: v.agent.clone(),
            id: k.to_owned(),
            login_time: v.login,
            exp_time: v.exp,
        })
    }
    Json(list)
}

#[debug_handler]
pub async fn user_post_remove_session(
    h: HeaderMap,
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = h.get("session_id");
    if session.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "session_id header is not set".to_string(),
        ));
    }
    let session = session.unwrap().to_str().unwrap().to_owned();
    let mut sessions = load_sessions().await;
    let our_session = sessions.get(&session);
    if our_session.is_none() {
        return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
    }
    let our_session = our_session.unwrap();
    if our_session.username != ext.username {
        return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
    }
    sessions.remove(&session);
    write_sessions(sessions).await;
    Ok(())
}

#[debug_handler]
pub async fn user_post_change_name(
    h: HeaderMap,
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = h.get("session_id");
    let newname = h.get("name");
    if session.is_none() || newname.is_none() {
        return Err((StatusCode::BAD_REQUEST, "Headers are not set".to_string()));
    }
    let session = session.unwrap().to_str().unwrap().to_owned();
    let newname = newname.unwrap().to_str().unwrap().to_owned();
    let newname = decode(&newname).unwrap();
    let mut sessions = load_sessions().await;
    let our_session = sessions.get(&session);
    if our_session.is_none() {
        return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
    }
    let our_session = our_session.unwrap();
    if our_session.username != ext.username {
        return Err((StatusCode::NOT_FOUND, "Session not found".to_string()));
    }
    sessions.insert(
        session,
        Session {
            agent: format!("{}", newname),
            ..our_session.to_owned()
        },
    );
    write_sessions(sessions).await;
    Ok(())
}

#[debug_handler]
pub async fn user_post_remove_all_session(
    ext: Extension<UserExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut sessions = load_sessions().await;
    for (k, v) in sessions.clone().iter() {
        if v.username != ext.username {
            continue;
        }
        sessions.remove(k).unwrap();
    }
    write_sessions(sessions).await;
    Ok(())
}
