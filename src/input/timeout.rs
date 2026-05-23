use std::time::{Duration, Instant};
use std::io::{self, Read, BufRead};

/// Wraps a reader and enforces a per-line read timeout.
pub struct TimeoutReader<R> {
    inner: R,
    timeout: Duration,
    started_at: Instant,
}

impl<R: BufRead> TimeoutReader<R> {
    pub fn new(inner: R, timeout: Duration) -> Self {
        Self {
            inner,
            timeout,
            started_at: Instant::now(),
        }
    }

    pub fn reset_timer(&mut self) {
        self.started_at = Instant::now();
    }

    pub fn is_timed_out(&self) -> bool {
        self.started_at.elapsed() >= self.timeout
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Read a line, returning `None` if timeout is exceeded before a newline.
    pub fn read_line_timeout(&mut self, buf: &mut String) -> io::Result<Option<usize>> {
        if self.is_timed_out() {
            return Ok(None);
        }
        self.reset_timer();
        let n = self.inner.read_line(buf)?;
        if self.is_timed_out() && n == 0 {
            Ok(None)
        } else {
            Ok(Some(n))
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> Read for TimeoutReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.is_timed_out() {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "read timeout exceeded"));
        }
        self.inner.read(buf)
    }
}

/// Configuration for timeout behaviour in the pipeline.
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub line_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            line_timeout: Duration::from_secs(5),
            idle_timeout: Duration::from_secs(30),
        }
    }
}
