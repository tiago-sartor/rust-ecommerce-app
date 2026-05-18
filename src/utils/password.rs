use crate::utils::errors::AppError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

/**
 * Hash a password before saving it to the database
 */
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Ok(hash.to_string()),
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

/**
 * Verify a user-provided password against its hash from the database
 */
pub fn verify_password(stored_password_hash: &str, provided_password: &str) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(stored_password_hash) {
        Ok(parsed_hash) => argon2.verify_password(provided_password.as_bytes(), &parsed_hash).is_ok(),
        Err(_) => false,
    }
}
