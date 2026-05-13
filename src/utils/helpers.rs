use crate::utils::errors::AppError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::distr::{Alphanumeric, SampleString};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tower_sessions::Session;

/**
 * Convert any serializable struct into a HashMap<String, String>
 */
pub fn struct_to_map<T: Serialize>(input: &T) -> HashMap<String, String> {
    let value = match serde_json::to_value(input) {
        Ok(v) => v,
        Err(_) => return HashMap::new(),
    };
    match value {
        Value::Object(obj) => obj
            .into_iter()
            .map(|(k, v)| match v {
                Value::String(s) => (k, s),
                _ => (k, v.to_string()),
            })
            .collect(),
        _ => HashMap::new(),
    }
}

/**
 * Random token generator
 */
pub fn generate_random_token(length: usize) -> String {
    return Alphanumeric.sample_string(&mut rand::rng(), length);
}

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

/**
 * Regenerate the session id and create a new CSRF token for the session
 */
pub async fn regenerate_session(session: &Session) -> Result<(), AppError> {
    session.cycle_id().await?;
    session.insert("csrf_token", generate_random_token(64)).await?;

    Ok(())
}
