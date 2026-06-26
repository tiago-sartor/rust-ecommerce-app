use crate::utils::AppError;
use base64::{Engine, engine::general_purpose::URL_SAFE as base64};
use rand::distr::{Alphanumeric, SampleString};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha512_256};
use sqlx::types::Decimal;
use std::collections::HashMap;
use time::OffsetDateTime;
use tower_sessions::Session;

/**
 * Format a standard UTC datetime to Brazil offset and format
 */
pub fn format_datetime_to_br(datetime: OffsetDateTime) -> String {
    const MONTHS_BR: [&str; 12] = ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"];
    let d = datetime.to_offset(time::macros::offset!(-3));

    format!(
        "{:02} {} {} {:02}:{:02}",
        d.day(),
        MONTHS_BR[d.month() as usize - 1],
        d.year(),
        d.hour(),
        d.minute()
    )
}

/**
 * Format a numeric value to BRL currency
 */
pub fn format_to_brl(value: Decimal) -> String {
    // Round to 2 decimal places using bankers rounding
    let abs_value = value.round_dp(2).abs();

    // Separate the integer part and the fractional part
    let integer_part = abs_value.trunc();
    let fractional_part = (abs_value.fract() * Decimal::new(100, 0)).trunc();

    // Format the integer part with thousands separators
    let int_str = integer_part.to_string();
    let mut formatted_int = String::new();
    let mut count = 0;

    // Iterate in reverse to place the thousands separator ('.')
    for c in int_str.chars().rev() {
        if count == 3 {
            formatted_int.push('.');
            count = 0;
        }
        formatted_int.push(c);
        count += 1;
    }

    // Reverse the string back to the correct order
    let formatted_int: String = formatted_int.chars().rev().collect();

    // Combine fragments into final BRL currency format
    let sign = if value.is_sign_negative() { "-" } else { "" };
    format!("{}R$ {},{:02}", sign, formatted_int, fractional_part)
}

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

/// Helper function to compute the SHA-512/256 hash as a base64 string
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha512_256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();

    base64.encode(result)
}

/**
 * Regenerate the session id and create a new CSRF token for the session
 */
pub async fn regenerate_session(session: &Session) -> Result<(), AppError> {
    session.cycle_id().await?;
    session.insert("csrf_token", generate_random_token(64)).await?;

    Ok(())
}
