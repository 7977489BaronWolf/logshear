//! Filtering and projection of extracted fields for output.

use super::field_extractor::ExtractedFields;

/// Configuration for field projection and filtering.
#[derive(Debug, Clone, Default)]
pub struct FieldFilter {
    /// If non-empty, only include these fields in output.
    pub include: Vec<String>,
    /// Fields to exclude from output.
    pub exclude: Vec<String>,
}

impl FieldFilter {
    pub fn new(include: Vec<String>, exclude: Vec<String>) -> Self {
        Self { include, exclude }
    }

    /// Returns true if no filtering is configured.
    pub fn is_passthrough(&self) -> bool {
        self.include.is_empty() && self.exclude.is_empty()
    }

    /// Apply the filter to extracted fields, returning a projected map.
    pub fn apply<'a>(
        &self,
        extracted: &'a ExtractedFields,
    ) -> Vec<(&'a str, &'a str)> {
        let mut pairs: Vec<(&str, &str)> = extracted
            .fields
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        // Sort for deterministic output
        pairs.sort_by_key(|(k, _)| *k);

        if !self.include.is_empty() {
            pairs.retain(|(k, _)| self.include.iter().any(|i| i == k));
            // Preserve include order
            pairs.sort_by_key(|(k, _)| {
                self.include.iter().position(|i| i == k).unwrap_or(usize::MAX)
            });
        }

        if !self.exclude.is_empty() {
            pairs.retain(|(k, _)| !self.exclude.iter().any(|e| e == k));
        }

        pairs
    }

    /// Format projected fields as a compact string.
    pub fn format_fields(&self, extracted: &ExtractedFields) -> String {
        if self.is_passthrough() && extracted.fields.is_empty() {
            return extracted.raw.clone();
        }
        let pairs = self.apply(extracted);
        if pairs.is_empty() {
            return extracted.raw.clone();
        }
        pairs
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
