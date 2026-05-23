//! Configuration for the tag injection feature.

use std::collections::HashMap;

/// Parsed tag configuration supplied via CLI or config file.
#[derive(Debug, Clone, Default)]
pub struct TagConfig {
    /// Static key=value pairs supplied by the user, e.g. `--tag env=prod`.
    pub tags: HashMap<String, String>,
    /// Automatically add a `_source` field with the log file path.
    pub inject_source: bool,
}

impl TagConfig {
    /// Parse a slice of `"key=value"` strings into a `TagConfig`.
    pub fn from_args(args: &[String], inject_source: bool) -> Result<Self, String> {
        let mut tags = HashMap::new();
        for arg in args {
            let mut parts = arg.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim().to_owned();
            let val = parts.next().unwrap_or("").trim().to_owned();
            if key.is_empty() {
                return Err(format!("Invalid tag (missing key): '{}'", arg));
            }
            tags.insert(key, val);
        }
        Ok(Self { tags, inject_source })
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty() && !self.inject_source
    }
}
