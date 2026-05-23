//! Schema inference for structured log fields.
//!
//! Inspects a sample of log lines and infers field types
//! (string, integer, float, boolean, timestamp) for JSON logs.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Boolean,
    Integer,
    Float,
    Timestamp,
    String,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub name: String,
    pub inferred_type: FieldType,
    pub nullable: bool,
    pub sample_count: usize,
}

#[derive(Debug, Default)]
pub struct Schema {
    pub fields: HashMap<String, FieldSchema>,
}

impl Schema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn infer_from_lines(&mut self, lines: &[&str]) {
        for line in lines {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(obj) = value.as_object() {
                    for (key, val) in obj {
                        let ft = infer_type(val);
                        let nullable = val.is_null();
                        let entry = self.fields.entry(key.clone()).or_insert_with(|| FieldSchema {
                            name: key.clone(),
                            inferred_type: ft.clone(),
                            nullable,
                            sample_count: 0,
                        });
                        entry.sample_count += 1;
                        if entry.inferred_type != ft && ft != FieldType::Unknown {
                            entry.inferred_type = FieldType::String;
                        }
                        if nullable {
                            entry.nullable = true;
                        }
                    }
                }
            }
        }
    }

    pub fn get(&self, field: &str) -> Option<&FieldSchema> {
        self.fields.get(field)
    }
}

fn infer_type(val: &serde_json::Value) -> FieldType {
    match val {
        serde_json::Value::Bool(_) => FieldType::Boolean,
        serde_json::Value::Number(n) => {
            if n.is_f64() && !n.is_i64() && !n.is_u64() {
                FieldType::Float
            } else {
                FieldType::Integer
            }
        }
        serde_json::Value::String(s) => {
            if looks_like_timestamp(s) {
                FieldType::Timestamp
            } else {
                FieldType::String
            }
        }
        serde_json::Value::Null => FieldType::Unknown,
        _ => FieldType::String,
    }
}

fn looks_like_timestamp(s: &str) -> bool {
    // Heuristic: ISO 8601 / RFC 3339 patterns
    s.len() >= 19
        && s.contains('T')
        && (s.contains('Z') || s.contains('+') || s.contains('-'))
        && s[..4].chars().all(|c| c.is_ascii_digit())
}
