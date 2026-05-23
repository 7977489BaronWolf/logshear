//! Tag injection: attach static or dynamic key-value tags to every log line.

use std::collections::HashMap;

/// A set of tags to attach to log records.
#[derive(Debug, Clone, Default)]
pub struct Tagger {
    static_tags: HashMap<String, String>,
    /// If true, a `_source` tag with the filename is injected automatically.
    pub inject_source: bool,
}

impl Tagger {
    pub fn new(static_tags: HashMap<String, String>, inject_source: bool) -> Self {
        Self { static_tags, inject_source }
    }

    /// Apply tags to a JSON object line.  Non-JSON lines are returned unchanged.
    /// Returns the (possibly modified) line.
    pub fn apply(&self, line: &str, source: Option<&str>) -> String {
        // Only attempt to tag JSON objects.
        let trimmed = line.trim();
        if !trimmed.starts_with('{') {
            return line.to_owned();
        }

        // Strip the trailing `}` and append tags.
        let body = trimmed.trim_end_matches('}').trim_end_matches(',');
        let mut parts: Vec<String> = Vec::new();

        for (k, v) in &self.static_tags {
            parts.push(format!(r#""{}":"{}"", k, v));
        }

        if self.inject_source {
            if let Some(src) = source {
                parts.push(format!(r#""_source":"{}"", src));
            }
        }

        if parts.is_empty() {
            return line.to_owned();
        }

        format!("{},{}}}", body, parts.join(","))
    }

    pub fn has_tags(&self) -> bool {
        !self.static_tags.is_empty() || self.inject_source
    }
}
