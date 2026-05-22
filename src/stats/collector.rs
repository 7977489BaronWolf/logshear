use std::time::{Duration, Instant};

/// Collects runtime statistics during pipeline execution.
#[derive(Debug, Default)]
pub struct StatsCollector {
    pub lines_read: u64,
    pub lines_matched: u64,
    pub lines_skipped: u64,
    pub bytes_read: u64,
    pub files_processed: u64,
    pub parse_errors: u64,
    start_time: Option<Instant>,
}

impl StatsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record_line(&mut self, bytes: u64, matched: bool) {
        self.lines_read += 1;
        self.bytes_read += bytes;
        if matched {
            self.lines_matched += 1;
        } else {
            self.lines_skipped += 1;
        }
    }

    pub fn record_parse_error(&mut self) {
        self.parse_errors += 1;
    }

    pub fn record_file(&mut self) {
        self.files_processed += 1;
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    pub fn match_rate(&self) -> f64 {
        if self.lines_read == 0 {
            return 0.0;
        }
        self.lines_matched as f64 / self.lines_read as f64 * 100.0
    }

    pub fn throughput_lines(&self) -> f64 {
        let secs = self.elapsed().as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.lines_read as f64 / secs
    }

    pub fn throughput_bytes(&self) -> f64 {
        let secs = self.elapsed().as_secs_f64();
        if secs == 0.0 {
            return 0.0;
        }
        self.bytes_read as f64 / secs
    }
}
