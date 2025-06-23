use database::repositories::{user::UserRepository, Repository};
use database::records::user::UserRecordMutation;
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{rand_core::OsRng, SaltString};

use crate::error::ApplicationError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserArgs {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn register_user(args: RegisterUserArgs) -> Result<(), crate::error::ApplicationError> {
    let user_repo = UserRepository::get().await;

    let existing_user = user_repo.find_by_email(
        args.email.as_str()
    ).await?;

    if existing_user.is_some() {
        return Err(ApplicationError::UserAlreadyExists(args.email));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(args.password.as_bytes(), &salt)
        .map_err(|e| ApplicationError::InternalError(format!("Failed to hash password: {}", e)))?
        .to_string();

    let user_mutation = UserRecordMutation {
        first_name: args.first_name,
        last_name: args.last_name,
        email: args.email,
        password: password_hash,
    };

    user_repo.create(user_mutation).await?;

    Ok(())
}
