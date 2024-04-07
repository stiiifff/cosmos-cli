use serde_json::Value;

pub(crate) fn scalar_to_plain(value: Value) -> String {
    match value {
        Value::Array(arr) => arr.into_iter().fold(String::new(), |acc, v| {
            (if !acc.is_empty() { acc + "\n" } else { acc }) + &scalar_to_plain(v)
        }),
        // Remove quotes from scalar values
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "".to_string(),
        // This function is only for scalar values
        Value::Object(_) => unimplemented!("Unsupported value type"),
    }
}
