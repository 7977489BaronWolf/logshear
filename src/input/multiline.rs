//! Multiline log record assembler.
//!
//! Joins continuation lines into a single logical record based on a
//! configurable pattern that identifies the *start* of a new record.

use regex::Regex;

/// Configuration for multiline joining.
#[derive(Debug, Clone)]
pub struct MultilineConfig {
    /// Regex that matches the first line of a new log record.
    pub start_pattern: Regex,
    /// Maximum number of lines to accumulate before flushing.
    pub max_lines: usize,
}

impl MultilineConfig {
    pub fn new(pattern: &str, max_lines: usize) -> Result<Self, regex::Error> {
        Ok(Self {
            start_pattern: Regex::new(pattern)?,
            max_lines,
        })
    }
}

/// Stateful assembler that buffers continuation lines.
#[derive(Debug)]
pub struct MultilineAssembler {
    config: MultilineConfig,
    buffer: Vec<String>,
}

impl MultilineAssembler {
    pub fn new(config: MultilineConfig) -> Self {
        Self {
            config,
            buffer: Vec::new(),
        }
    }

    /// Feed a single raw line. Returns a completed record when one is ready.
    pub fn push(&mut self, line: String) -> Option<String> {
        let is_start = self.config.start_pattern.is_match(&line);

        if is_start && !self.buffer.is_empty() {
            let record = self.flush();
            self.buffer.push(line);
            return Some(record);
        }

        self.buffer.push(line);

        // Safety valve: flush when buffer is too large.
        if self.buffer.len() >= self.config.max_lines {
            return Some(self.flush());
        }

        None
    }

    /// Signal end-of-stream; returns any remaining buffered content.
    pub fn finish(&mut self) -> Option<String> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.flush())
        }
    }

    fn flush(&mut self) -> String {
        let record = self.buffer.join("\n");
        self.buffer.clear();
        record
    }

    /// Drain all complete records from an iterator of lines.
    pub fn process_lines<I: IntoIterator<Item = String>>(
        &mut self,
        lines: I,
    ) -> Vec<String> {
        let mut out = Vec::new();
        for line in lines {
            if let Some(record) = self.push(line) {
                out.push(record);
            }
        }
        if let Some(record) = self.finish() {
            out.push(record);
        }
        out
    }
}
