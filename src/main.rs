use axum::{Router, routing::get};
use tracing::info;

mod infrastructure;
mod middleware;
mod trades_api;

use infrastructure::get_connection_pool;
use trades_api::health;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let epex = get_connection_pool("CONNECTION_STRINGS__PG__EPEX_M7").await;
    let nordpool = get_connection_pool("CONNECTION_STRINGS__PG__NORDPOOL_INTRADAY").await;

    let app = Router::new().route("/health", get(health));

    let port = "8000";
    let address = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("failed to bind to port 8000");

    info!("server starting on {address}");

    axum::serve(listener, app).await.expect("server failed");
}
