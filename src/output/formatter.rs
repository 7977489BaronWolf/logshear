use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Raw,
    Json,
    Fields(Vec<String>),
}

pub struct Formatter {
    format: OutputFormat,
    color: bool,
}

impl Formatter {
    pub fn new(format: OutputFormat, color: bool) -> Self {
        Self { format, color }
    }

    pub fn format_line(&self, raw: &str, fields: &HashMap<String, Value>) -> Option<String> {
        match &self.format {
            OutputFormat::Raw => Some(raw.to_string()),
            OutputFormat::Json => {
                if fields.is_empty() {
                    Some(raw.to_string())
                } else {
                    serde_json::to_string(fields).ok()
                }
            }
            OutputFormat::Fields(keys) => {
                let parts: Vec<String> = keys
                    .iter()
                    .filter_map(|k| fields.get(k).map(|v| format_value(v)))
                    .collect();
                if parts.is_empty() {
                    None
                } else {
                    Some(parts.join("\t"))
                }
            }
        }
    }

    pub fn highlight(&self, line: &str, term: &str) -> String {
        if !self.color || term.is_empty() {
            return line.to_string();
        }
        line.replace(term, &format!("\x1b[1;33m{}\x1b[0m", term))
    }
}

fn format_value(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::from("null"),
        other => other.to_string(),
    }
}
