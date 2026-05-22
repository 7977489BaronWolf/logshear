use std::io::{self, BufWriter, Write};

pub struct OutputWriter {
    inner: BufWriter<Box<dyn Write>>,
    count: usize,
    limit: Option<usize>,
}

impl OutputWriter {
    pub fn stdout(limit: Option<usize>) -> Self {
        Self {
            inner: BufWriter::new(Box::new(io::stdout())),
            count: 0,
            limit,
        }
    }

    pub fn from_writer(w: Box<dyn Write>, limit: Option<usize>) -> Self {
        Self {
            inner: BufWriter::new(w),
            count: 0,
            limit,
        }
    }

    /// Write a line to output. Returns `false` if the limit has been reached.
    pub fn write_line(&mut self, line: &str) -> io::Result<bool> {
        if let Some(lim) = self.limit {
            if self.count >= lim {
                return Ok(false);
            }
        }
        writeln!(self.inner, "{}", line)?;
        self.count += 1;
        Ok(true)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }

    pub fn lines_written(&self) -> usize {
        self.count
    }

    pub fn limit_reached(&self) -> bool {
        self.limit.map_or(false, |lim| self.count >= lim)
    }
}
