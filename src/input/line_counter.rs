/// Tracks line numbers and byte offsets as lines are consumed from input sources.
/// Useful for error reporting and progress indication during log processing.

#[derive(Debug, Clone, Default)]
pub struct LineCounter {
    pub line: u64,
    pub byte_offset: u64,
    pub source: Option<String>,
}

impl LineCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_source(source: impl Into<String>) -> Self {
        Self {
            source: Some(source.into()),
            ..Default::default()
        }
    }

    /// Record a line being consumed. `len` is the byte length including newline.
    pub fn advance(&mut self, len: usize) {
        self.line += 1;
        self.byte_offset += len as u64;
    }

    /// Reset counters, optionally changing the source label.
    pub fn reset(&mut self, source: Option<String>) {
        self.line = 0;
        self.byte_offset = 0;
        if source.is_some() {
            self.source = source;
        }
    }

    /// Human-readable position string, e.g. "file.log:42".
    pub fn position_label(&self) -> String {
        match &self.source {
            Some(src) => format!("{}:{}", src, self.line),
            None => format!("line {}", self.line),
        }
    }

    pub fn is_at_start(&self) -> bool {
        self.line == 0
    }
}

impl std::fmt::Display for LineCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position_label())
    }
}
