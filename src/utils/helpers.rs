use crate::utils::AppError;
use rand::distr::{Alphanumeric, SampleString};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;
use tower_sessions::Session;

/**
 * Format a standard UTC datetime to Brazil offset and format
 */
pub fn format_datetime_to_br(datetime: OffsetDateTime) -> String {
    const MONTHS_BR: [&str; 12] = [
        "Jan", "Fev", "Mar", "Abr", "Mai", "Jun", 
        "Jul", "Ago", "Set", "Out", "Nov", "Dez"
    ];
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
 * Regenerate the session id and create a new CSRF token for the session
 */
pub async fn regenerate_session(session: &Session) -> Result<(), AppError> {
    session.cycle_id().await?;
    session.insert("csrf_token", generate_random_token(64)).await?;

    Ok(())
}
