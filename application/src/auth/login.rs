use database::repositories::{user::UserRepository, Repository};
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordVerifier};
use argon2::password_hash::PasswordHash;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::error;

use crate::error::ApplicationError;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserArgs {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    email: String,
    exp: u64,
    iat: u64,
}

pub async fn log_user_in(args: LoginUserArgs) -> Result<LoginResponse, ApplicationError> {
    let user_repo = UserRepository::get().await;

    let user = user_repo.find_by_email(&args.email).await?;
    let user = user.ok_or_else(|| ApplicationError::InvalidInput("Invalid email or password".to_string()))?;

    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(&user.password)
        .map_err(|e| ApplicationError::InternalError(format!("Failed to parse password hash: {}", e)))?;
    
    argon2
        .verify_password(args.password.as_bytes(), &password_hash)
        .map_err(|_| ApplicationError::InvalidInput("Invalid email or password".to_string()))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ApplicationError::InternalError(format!("Failed to get current time: {}", e)))?
        .as_secs();

    let expiration = now + (24 * 60 * 60);

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: expiration,
        iat: now,
    };

    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        error!("JWT_SECRET environment variable not set");
        ApplicationError::InternalError("There was an internal error".to_string())
    })?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    )
    .map_err(|e| ApplicationError::InternalError(format!("Failed to create JWT: {}", e)))?;

    let user_info = UserInfo {
        id: user.id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };

    Ok(LoginResponse {
        token,
        user: user_info,
    })
}
