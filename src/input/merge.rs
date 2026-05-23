//! Line merging: combine consecutive lines matching a pattern into one record.

use regex::Regex;

/// Configuration for merging consecutive log lines.
#[derive(Debug, Clone)]
pub struct MergeConfig {
    /// Regex that, when matched on a line, signals it should be merged
    /// into the *previous* line (continuation line).
    pub continuation_pattern: Regex,
    /// Separator inserted between merged lines.
    pub separator: String,
    /// Maximum number of lines that can be merged into one record.
    pub max_lines: usize,
}

impl MergeConfig {
    pub fn new(pattern: &str, separator: &str, max_lines: usize) -> Result<Self, regex::Error> {
        Ok(Self {
            continuation_pattern: Regex::new(pattern)?,
            separator: separator.to_string(),
            max_lines,
        })
    }
}

/// Stateful merger that accumulates continuation lines.
pub struct LineMerger {
    config: MergeConfig,
    pending: Option<String>,
    merge_count: usize,
}

impl LineMerger {
    pub fn new(config: MergeConfig) -> Self {
        Self {
            config,
            pending: None,
            merge_count: 0,
        }
    }

    /// Feed a line; returns a completed record if one is ready.
    pub fn push(&mut self, line: String) -> Option<String> {
        let is_continuation = self.config.continuation_pattern.is_match(&line);

        if is_continuation && self.pending.is_some() && self.merge_count < self.config.max_lines {
            let buf = self.pending.as_mut().unwrap();
            buf.push_str(&self.config.separator);
            buf.push_str(&line);
            self.merge_count += 1;
            None
        } else {
            // Flush any pending record, start a new one.
            let completed = self.pending.replace(line);
            self.merge_count = 0;
            completed
        }
    }

    /// Flush any remaining buffered record (call at end of input).
    pub fn flush(&mut self) -> Option<String> {
        self.merge_count = 0;
        self.pending.take()
    }
}
