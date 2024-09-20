use slekup_api::{app, config::Config, tracing::init_tracing};
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load environment variables");

    let config = Config::init();

    init_tracing();

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .expect("Failed to start TCP server");

    println!("her");
    info!("🔗 Listening on: {}", listener.local_addr().unwrap());
    info!("🚀 Slekup API started successfully");

    axum::serve(listener, app(config).await)
        .await
        .expect("Failed to start axum server");

    println!("finishing for some reason?")
}
