use axum::Router;

pub mod health;
pub mod auth;

pub fn router() -> Router {
    Router::new()
        .route("/healthcheck", axum::routing::get(health::health_check))
        .route("/auth/register", axum::routing::post(auth::register))
        .route("/auth/login", axum::routing::post(auth::login))
} 