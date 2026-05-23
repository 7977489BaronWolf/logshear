//! Line truncation support for long log lines.

use std::borrow::Cow;

/// Configuration for line truncation behavior.
#[derive(Debug, Clone)]
pub struct TruncateConfig {
    /// Maximum number of bytes allowed per line.
    pub max_bytes: usize,
    /// Suffix appended to truncated lines to indicate truncation.
    pub suffix: String,
}

impl Default for TruncateConfig {
    fn default() -> Self {
        Self {
            max_bytes: 8192,
            suffix: String::from("...[truncated]"),
        }
    }
}

/// Truncates a line to the configured maximum byte length, appending a suffix
/// if truncation occurs. Truncation is performed on a UTF-8 character boundary
/// to avoid producing invalid strings.
pub fn truncate_line<'a>(line: &'a str, config: &TruncateConfig) -> Cow<'a, str> {
    if line.len() <= config.max_bytes {
        return Cow::Borrowed(line);
    }

    let suffix_len = config.suffix.len();
    let target = config.max_bytes.saturating_sub(suffix_len);

    // Find the largest valid UTF-8 boundary at or before `target`.
    let cut = floor_char_boundary(line, target);
    let mut truncated = String::with_capacity(cut + suffix_len);
    truncated.push_str(&line[..cut]);
    truncated.push_str(&config.suffix);
    Cow::Owned(truncated)
}

/// Returns the largest byte index `<= max` that is a valid UTF-8 char boundary.
fn floor_char_boundary(s: &str, max: usize) -> usize {
    if max >= s.len() {
        return s.len();
    }
    let mut idx = max;
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

/// Applies truncation to every line in an iterator, yielding owned strings.
pub struct TruncateIter<I> {
    inner: I,
    config: TruncateConfig,
}

impl<I> TruncateIter<I> {
    pub fn new(inner: I, config: TruncateConfig) -> Self {
        Self { inner, config }
    }
}

impl<I: Iterator<Item = String>> Iterator for TruncateIter<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|line| {
            truncate_line(&line, &self.config).into_owned()
        })
    }
}
