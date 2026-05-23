//! Batch reader: groups log lines into fixed-size or time-bounded batches
//! for more efficient downstream processing.

use std::time::{Duration, Instant};

/// A batch of log lines collected from the input stream.
#[derive(Debug, Clone, Default)]
pub struct Batch {
    pub lines: Vec<String>,
    pub created_at: Option<Instant>,
}

impl Batch {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            created_at: Some(Instant::now()),
        }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn age(&self) -> Option<Duration> {
        self.created_at.map(|t| t.elapsed())
    }
}

/// Configuration for the batch collector.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of lines per batch.
    pub max_size: usize,
    /// Maximum age of a batch before it is flushed regardless of size.
    pub max_age: Duration,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_size: 256,
            max_age: Duration::from_millis(200),
        }
    }
}

/// Accumulates lines into batches according to `BatchConfig`.
pub struct BatchCollector {
    config: BatchConfig,
    current: Batch,
}

impl BatchCollector {
    pub fn new(config: BatchConfig) -> Self {
        Self {
            config,
            current: Batch::new(),
        }
    }

    /// Add a line. Returns a completed batch if a flush threshold is reached.
    pub fn add(&mut self, line: String) -> Option<Batch> {
        self.current.push(line);
        if self.should_flush() {
            Some(self.flush())
        } else {
            None
        }
    }

    /// Force-flush the current batch regardless of thresholds.
    pub fn flush(&mut self) -> Batch {
        let mut batch = Batch::new();
        std::mem::swap(&mut batch, &mut self.current);
        batch
    }

    /// Returns true if the current batch should be flushed.
    pub fn should_flush(&self) -> bool {
        if self.current.len() >= self.config.max_size {
            return true;
        }
        if let Some(age) = self.current.age() {
            if age >= self.config.max_age {
                return true;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.current.is_empty()
    }
}
