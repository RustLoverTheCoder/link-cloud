use link_cloud::{app, config::PORT};
use std::{net::SocketAddr, time::Instant};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let instant = Instant::now();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));
    tracing::debug!("listening on {}", addr);
    let server = axum::Server::bind(&addr).serve(app().into_make_service());
    tracing::info!("Started Server in {:.3?}", instant.elapsed());
    if let Err(err) = server.await {
        tracing::error!("Server error: {:?}", err)
    }
}
