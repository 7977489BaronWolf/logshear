//! Rate limiter for controlling log line throughput during processing.
//!
//! Useful when tailing live logs or reading from fast sources to avoid
//! overwhelming downstream consumers.

use std::time::{Duration, Instant};

/// Controls the rate at which log lines are processed.
pub struct RateLimiter {
    /// Maximum lines per second (0 = unlimited)
    max_lines_per_sec: u64,
    /// Lines processed in the current window
    lines_in_window: u64,
    /// Start of the current 1-second window
    window_start: Instant,
    /// Total lines passed through
    total_lines: u64,
    /// Total time spent sleeping (nanoseconds)
    total_sleep_ns: u64,
}

impl RateLimiter {
    /// Create a new rate limiter. Pass `0` for unlimited throughput.
    pub fn new(max_lines_per_sec: u64) -> Self {
        Self {
            max_lines_per_sec,
            lines_in_window: 0,
            window_start: Instant::now(),
            total_lines: 0,
            total_sleep_ns: 0,
        }
    }

    /// Returns true if rate limiting is active.
    pub fn is_active(&self) -> bool {
        self.max_lines_per_sec > 0
    }

    /// Call once per line. May sleep to enforce the rate limit.
    pub fn throttle(&mut self) {
        if self.max_lines_per_sec == 0 {
            self.total_lines += 1;
            return;
        }

        let elapsed = self.window_start.elapsed();

        // Reset window every second
        if elapsed >= Duration::from_secs(1) {
            self.lines_in_window = 0;
            self.window_start = Instant::now();
        }

        self.lines_in_window += 1;
        self.total_lines += 1;

        // If we've exceeded the limit within the window, sleep for the remainder
        if self.lines_in_window > self.max_lines_per_sec {
            let window_end = self.window_start + Duration::from_secs(1);
            let now = Instant::now();
            if window_end > now {
                let sleep_dur = window_end - now;
                self.total_sleep_ns += sleep_dur.as_nanos() as u64;
                std::thread::sleep(sleep_dur);
                // Reset after sleeping into the next window
                self.lines_in_window = 0;
                self.window_start = Instant::now();
            }
        }
    }

    /// Total lines processed.
    pub fn total_lines(&self) -> u64 {
        self.total_lines
    }

    /// Total nanoseconds spent sleeping due to rate limiting.
    pub fn total_sleep_ns(&self) -> u64 {
        self.total_sleep_ns
    }

    /// Effective lines/sec since creation (approximate).
    pub fn effective_rate(&self) -> f64 {
        let elapsed = self.window_start.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.lines_in_window as f64 / elapsed
        } else {
            0.0
        }
    }
}
