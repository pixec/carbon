use std::net::SocketAddr;

use axum::{Router, Server};
use log::{error, info};

mod routes;
mod server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let manager = server::Manager::new();

    info!("Initializing server manager...");
    manager.initialize().unwrap_or_else(|err| {
        error!("Failed to initialize server manager: {}", err);
    });

    let router = Router::new().nest("/system", routes::system::routing());

    info!("Starting HTTP server...");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap_or_else(|err| {
            error!("Failed to start HTTP server: {}", err);
        });
}
