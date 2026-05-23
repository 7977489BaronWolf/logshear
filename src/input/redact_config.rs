//! Serializable configuration for the redaction module.

use serde::{Deserialize, Serialize};

/// Top-level redaction configuration, typically loaded from a config file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedactConfig {
    /// Field names whose values should be fully replaced.
    #[serde(default)]
    pub fields: Vec<String>,

    /// Regex patterns; matching substrings in any value are replaced.
    #[serde(default)]
    pub patterns: Vec<String>,

    /// Replacement string. Defaults to `[REDACTED]`.
    #[serde(default = "default_placeholder")]
    pub placeholder: String,
}

fn default_placeholder() -> String {
    "[REDACTED]".to_string()
}

impl RedactConfig {
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.patterns.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_placeholder_value() {
        let cfg = RedactConfig::default();
        assert_eq!(cfg.placeholder, "[REDACTED]");
    }

    #[test]
    fn is_empty_when_no_rules() {
        let cfg = RedactConfig::default();
        assert!(cfg.is_empty());
    }

    #[test]
    fn not_empty_with_field() {
        let cfg = RedactConfig {
            fields: vec!["password".into()],
            ..Default::default()
        };
        assert!(!cfg.is_empty());
    }
}
