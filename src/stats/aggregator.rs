use super::StatsCollector;

/// Merges statistics from multiple collectors (e.g. parallel workers).
#[derive(Debug, Default)]
pub struct Aggregator {
    collectors: Vec<StatsCollector>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, collector: StatsCollector) {
        self.collectors.push(collector);
    }

    /// Merge all collected stats into a single `StatsCollector`.
    /// The elapsed time is taken from the longest-running collector.
    pub fn merge(&self) -> StatsCollector {
        let mut merged = StatsCollector::new();
        let mut max_elapsed = std::time::Duration::ZERO;

        for c in &self.collectors {
            merged.lines_read += c.lines_read;
            merged.lines_matched += c.lines_matched;
            merged.lines_skipped += c.lines_skipped;
            merged.bytes_read += c.bytes_read;
            merged.files_processed += c.files_processed;
            merged.parse_errors += c.parse_errors;

            let elapsed = c.elapsed();
            if elapsed > max_elapsed {
                max_elapsed = elapsed;
            }
        }

        // Simulate a start time that gives us the correct elapsed duration.
        merged.start_time = Some(
            std::time::Instant::now()
                .checked_sub(max_elapsed)
                .unwrap_or(std::time::Instant::now()),
        );
        merged
    }

    pub fn collector_count(&self) -> usize {
        self.collectors.len()
    }
}
