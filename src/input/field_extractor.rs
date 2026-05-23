//! Field extraction from structured (JSON) and unstructured log lines.

use std::collections::HashMap;

/// Represents extracted fields from a log line.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedFields {
    pub fields: HashMap<String, String>,
    pub raw: String,
}

impl ExtractedFields {
    pub fn new(raw: String) -> Self {
        Self {
            fields: HashMap::new(),
            raw,
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(|s| s.as_str())
    }
}

/// Attempts to extract fields from a log line.
/// Tries JSON first, then falls back to key=value pattern.
pub fn extract_fields(line: &str) -> ExtractedFields {
    let mut result = ExtractedFields::new(line.to_string());

    if let Some(fields) = try_extract_json(line) {
        result.fields = fields;
        return result;
    }

    if let Some(fields) = try_extract_kv(line) {
        result.fields = fields;
    }

    result
}

fn try_extract_json(line: &str) -> Option<HashMap<String, String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('{') {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(trimmed).ok()?;
    let obj = value.as_object()?;
    let mut map = HashMap::new();
    for (k, v) in obj {
        let s = match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        map.insert(k.clone(), s);
    }
    Some(map)
}

fn try_extract_kv(line: &str) -> Option<HashMap<String, String>> {
    let mut map = HashMap::new();
    // Match patterns like key=value or key="value with spaces"
    let re = regex::Regex::new(r#"(\w+)=(?:"([^"]*)"|([^\s,]+))"#).ok()?;
    for cap in re.captures_iter(line) {
        let key = cap[1].to_string();
        let val = cap
            .get(2)
            .or_else(|| cap.get(3))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        map.insert(key, val);
    }
    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}
