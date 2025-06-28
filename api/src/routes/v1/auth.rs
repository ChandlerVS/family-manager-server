use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use application::auth::register::{register_user, RegisterUserArgs};
use application::auth::login::{log_user_in, LoginUserArgs};
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

pub async fn login(Json(args): Json<LoginUserArgs>) -> Response {
    let result = log_user_in(args).await;

    match result {
        Ok(login_response) => (
            StatusCode::OK,
            Json(login_response)
        ).into_response(),
        Err(e) => e.into_response()
    }
}
