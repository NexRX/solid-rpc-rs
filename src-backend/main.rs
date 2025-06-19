use std::net::SocketAddr;

use axum_embed::{FallbackBehavior, ServeEmbed};
use qubit::{Router, handler};
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[handler(query)]
async fn hello_world() -> String {
    "Hello, world!".to_string()
}

const BINDINGS_DIR: &str = "./bindings";

#[derive(RustEmbed, Clone)]
#[folder = "dist/"]
struct Spa;

#[tokio::main]
async fn main() {
    // Construct the qubit router
    let router = router();

    // Save the type
    router.write_bindings_to_dir(BINDINGS_DIR);
    println!("Generated Bindings: {BINDINGS_DIR}");

    // Spa
    let spa_service = ServeEmbed::<Spa>::with_parameters(
        Some("/index.html".to_string()),
        FallbackBehavior::Ok,
        Some("/index.html".to_string()),
    );

    // Create service and handle
    let (qubit_service, qubit_handle) = router.to_service(());

    // Nest into an Axum rouer
    let axum_router = axum::Router::<()>::new()
        .nest_service("/rpc", qubit_service)
        .fallback_service(spa_service) // Improvement, if /asset folder, then 404 if resource not found with no body
        .layer(CorsLayer::permissive()); // Improvement, remove on release builds

    // Start a Hyper server
    println!("Listening at 127.0.0.1:9944");
    axum::serve(
        TcpListener::bind(&SocketAddr::from(([127, 0, 0, 1], 9944)))
            .await
            .unwrap(),
        axum_router,
    )
    .await
    .unwrap();

    // Shutdown Qubit
    qubit_handle.stop().unwrap();
}

fn router() -> qubit::Router<()> {
    Router::new().handler(hello_world)
}

#[cfg(test)] // Allows
#[test]
fn generate_bindings() {
    router().write_bindings_to_dir(BINDINGS_DIR);
    println!("Generated Bindings: {BINDINGS_DIR}");
}
