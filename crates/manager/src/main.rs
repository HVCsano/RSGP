use std::error::Error;

use axum::{Router, routing::get};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod config;
mod endpoints;
mod jwt;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    info!("=== RSGP Manager ===");
    info!("Loading configs...");
    config::loader::load_configs().await;
    info!("Config loading done, starting server...");
    let app = Router::new()
        .route(
            "/",
            get(|| async move {
                "RSGP Manager API is running. Use the web interface to interact with it."
            }),
        )
        .route("/auth/login", get(endpoints::login::login))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());
    info!("[API] Starting server on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
