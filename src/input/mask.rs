//! Field masking: partially obscure field values (e.g. show only first/last N chars).

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MaskConfig {
    /// Map of field name → mask rule.
    pub fields: HashMap<String, MaskRule>,
}

#[derive(Debug, Clone)]
pub enum MaskRule {
    /// Keep the first `n` characters, replace the rest with `*`.
    Prefix(usize),
    /// Keep the last `n` characters, replace the leading portion with `*`.
    Suffix(usize),
    /// Replace the entire value with a fixed placeholder.
    Full(String),
}

impl MaskConfig {
    pub fn new(fields: HashMap<String, MaskRule>) -> Self {
        Self { fields }
    }

    /// Apply masking rules to a mutable map of fields extracted from a log line.
    pub fn apply(&self, record: &mut HashMap<String, String>) {
        for (field, rule) in &self.fields {
            if let Some(value) = record.get_mut(field) {
                *value = apply_rule(value, rule);
            }
        }
    }

    /// Apply masking to a raw line by scanning for `key=value` or `"key":"value"` patterns.
    /// Returns the modified line.
    pub fn apply_to_line(&self, line: &str) -> String {
        let mut result = line.to_string();
        for (field, rule) in &self.fields {
            result = mask_in_line(&result, field, rule);
        }
        result
    }
}

fn apply_rule(value: &str, rule: &MaskRule) -> String {
    match rule {
        MaskRule::Prefix(n) => {
            let keep = (*n).min(value.len());
            let masked = "*".repeat(value.len().saturating_sub(keep));
            format!("{}{}", &value[..keep], masked)
        }
        MaskRule::Suffix(n) => {
            let total = value.len();
            let keep = (*n).min(total);
            let masked = "*".repeat(total.saturating_sub(keep));
            format!("{}{}", masked, &value[total - keep..])
        }
        MaskRule::Full(placeholder) => placeholder.clone(),
    }
}

fn mask_in_line(line: &str, field: &str, rule: &MaskRule) -> String {
    // Support simple `field=value` (unquoted, space/comma terminated) patterns.
    let prefix = format!("{}=", field);
    if let Some(start) = line.find(&prefix) {
        let val_start = start + prefix.len();
        let val_end = line[val_start..]
            .find(|c: char| c == ' ' || c == ',' || c == '}')
            .map(|i| val_start + i)
            .unwrap_or(line.len());
        let original = &line[val_start..val_end];
        let masked = apply_rule(original, rule);
        return format!("{}{}{}{}", &line[..val_start], masked, &line[val_end..], "");
    }
    line.to_string()
}
