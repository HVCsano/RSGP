use std::error::Error;

use axum::{
    Router,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::endpoints::setup::setup_worker;

mod conf;
mod endpoints;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    info!("=== RSGP Worker v{} ===", env!("CARGO_PKG_VERSION"));
    info!("Loading configs...");
    conf::loader::load_configs().await;
    info!("Config loading done, starting api...");
    let app = Router::new()
        .route(
            "/",
            get(|| async move {
                "RSGP Worker API is running. Use the manager web interface to interact with it."
            }),
        )
        .route("/setup", post(setup_worker))
        .nest("/a", endpoints::routes())
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());
    info!("[API] Starting server on :3100");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
