use axum::Router;
use dotenv::dotenv;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod routes;

#[tokio::main]
async fn main() {
    if !cfg!(feature = "production") {
        dotenv().ok();
    }

    let environment = std::env::var("FAMILY_ENV").unwrap_or_else(|_| "production".to_string());

    let _subscriber = if environment == "production" {
        FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .with_line_number(false)
            .with_file(false)
            .compact()
            .init()
    } else {
        FmtSubscriber::builder()
            .with_max_level(Level::DEBUG)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(true)
            .pretty()
            .init()
    };

    info!("Starting server in {} environment", environment);

    let app = Router::new()
        .nest("/api/v1", routes::v1::router());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    info!("Server is running on port {}", port);
    axum::serve(listener, app).await.unwrap();
}
