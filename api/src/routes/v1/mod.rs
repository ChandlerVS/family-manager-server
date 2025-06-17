use axum::Router;

pub mod health;

pub fn router() -> Router {
    Router::new()
        .route("/healthcheck", axum::routing::get(health::health_check))
} 