use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use application::auth::register::{register_user, RegisterUserArgs};
use serde_json::json;

pub async fn register(Json(args): Json<RegisterUserArgs>) -> Response {
    let result = register_user(args).await;

    match result {
        Ok(()) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "User registered successfully"
            }))
        ).into_response(),
        Err(e) => e.into_response()
    }
} 