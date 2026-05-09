use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Convert any serializable struct into a HashMap<String, String>
pub fn struct_to_map<T: Serialize>(input: &T) -> HashMap<String, String> {
    let value = serde_json::to_value(input).unwrap();
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
