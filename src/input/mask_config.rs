//! Deserializable configuration for field masking rules.

use std::collections::HashMap;
use crate::input::mask::{MaskConfig, MaskRule};

#[derive(Debug, Clone)]
pub struct RawMaskEntry {
    pub mode: String,   // "prefix" | "suffix" | "full"
    pub n: Option<usize>,
    pub placeholder: Option<String>,
}

impl RawMaskEntry {
    pub fn new_prefix(n: usize) -> Self {
        Self { mode: "prefix".into(), n: Some(n), placeholder: None }
    }

    pub fn new_suffix(n: usize) -> Self {
        Self { mode: "suffix".into(), n: Some(n), placeholder: None }
    }

    pub fn new_full(placeholder: impl Into<String>) -> Self {
        Self { mode: "full".into(), n: None, placeholder: Some(placeholder.into()) }
    }

    pub fn into_rule(self) -> Result<MaskRule, String> {
        match self.mode.as_str() {
            "prefix" => {
                let n = self.n.ok_or("prefix mode requires `n`")?;
                Ok(MaskRule::Prefix(n))
            }
            "suffix" => {
                let n = self.n.ok_or("suffix mode requires `n`")?;
                Ok(MaskRule::Suffix(n))
            }
            "full" => {
                let ph = self.placeholder.unwrap_or_else(|| "[MASKED]".into());
                Ok(MaskRule::Full(ph))
            }
            other => Err(format!("unknown mask mode: {}", other)),
        }
    }
}

/// Build a `MaskConfig` from a map of field-name → raw entry.
pub fn build_mask_config(
    entries: HashMap<String, RawMaskEntry>,
) -> Result<MaskConfig, String> {
    let mut fields = HashMap::new();
    for (field, entry) in entries {
        let rule = entry.into_rule()?;
        fields.insert(field, rule);
    }
    Ok(MaskConfig::new(fields))
}
