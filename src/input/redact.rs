//! Field redaction for sensitive log data.
//!
//! Supports pattern-based redaction (regex) and field-name-based redaction,
//! replacing matched content with a configurable placeholder.

use regex::Regex;
use std::collections::HashSet;

/// Configuration for a single redaction rule.
#[derive(Debug, Clone)]
pub enum RedactRule {
    /// Redact any field whose name is in this set.
    FieldName(HashSet<String>),
    /// Redact substrings matching this pattern in field values.
    Pattern(Regex),
}

/// Redactor applies a set of rules to log line fields or raw text.
#[derive(Debug, Clone)]
pub struct Redactor {
    rules: Vec<RedactRule>,
    placeholder: String,
}

impl Redactor {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            rules: Vec::new(),
            placeholder: placeholder.into(),
        }
    }

    pub fn add_field_names(&mut self, names: impl IntoIterator<Item = impl Into<String>>) {
        let set: HashSet<String> = names.into_iter().map(|s| s.into()).collect();
        self.rules.push(RedactRule::FieldName(set));
    }

    pub fn add_pattern(&mut self, pattern: &str) -> Result<(), regex::Error> {
        let re = Regex::new(pattern)?;
        self.rules.push(RedactRule::Pattern(re));
        Ok(())
    }

    /// Redact a raw string value using pattern rules.
    pub fn redact_value(&self, value: &str) -> String {
        let mut result = value.to_string();
        for rule in &self.rules {
            if let RedactRule::Pattern(re) = rule {
                result = re.replace_all(&result, self.placeholder.as_str()).into_owned();
            }
        }
        result
    }

    /// Returns true if the given field name should be fully redacted.
    pub fn should_redact_field(&self, field: &str) -> bool {
        for rule in &self.rules {
            if let RedactRule::FieldName(names) = rule {
                if names.contains(field) {
                    return true;
                }
            }
        }
        false
    }

    /// Redact a JSON-like key/value pair, returning the (possibly redacted) value.
    pub fn redact_field(&self, key: &str, value: &str) -> String {
        if self.should_redact_field(key) {
            self.placeholder.clone()
        } else {
            self.redact_value(value)
        }
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }
}
