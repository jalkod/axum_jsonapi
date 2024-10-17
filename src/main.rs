use std::net::SocketAddr;

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use log::info;
use simple_logger::SimpleLogger;

pub mod json_api;
pub mod routes;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();
    let app = Router::new()
        .route("/fib", get(routes::fib))
        .route("/errors", get(routes::errors));

    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3443));

    info!("Starting HTTPS server on {:?}...", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
