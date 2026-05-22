use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use super::encoding::{decode_line, detect_encoding, strip_bom, Encoding};

/// Options controlling how lines are read from a source.
#[derive(Debug, Clone)]
pub struct ReaderOptions {
    /// Encoding override; `None` means auto-detect.
    pub encoding: Option<Encoding>,
    /// Maximum line length in bytes before truncation.
    pub max_line_bytes: usize,
}

impl Default for ReaderOptions {
    fn default() -> Self {
        Self {
            encoding: None,
            max_line_bytes: 1024 * 1024,
        }
    }
}

/// Reads lines from a file, handling encoding detection and BOM stripping.
pub struct LineReader {
    inner: BufReader<Box<dyn Read>>,
    encoding: Encoding,
    max_line_bytes: usize,
}

impl LineReader {
    /// Opens a file for line-by-line reading with the given options.
    pub fn open(path: &Path, opts: &ReaderOptions) -> io::Result<Self> {
        let file = File::open(path)?;
        Self::from_reader(Box::new(file), opts)
    }

    /// Wraps any `Read` source.
    pub fn from_reader(reader: Box<dyn Read>, opts: &ReaderOptions) -> io::Result<Self> {
        let mut buf_reader = BufReader::new(reader);
        // Peek at first bytes to detect encoding / BOM
        let peek = buf_reader.fill_buf()?;
        let encoding = opts
            .encoding
            .clone()
            .unwrap_or_else(|| detect_encoding(peek));
        // Consume BOM bytes if present
        let bom_len = {
            let stripped = strip_bom(peek);
            peek.len() - stripped.len()
        };
        if bom_len > 0 {
            buf_reader.consume(bom_len);
        }
        Ok(Self {
            inner: buf_reader,
            encoding,
            max_line_bytes: opts.max_line_bytes,
        })
    }

    /// Returns the detected encoding.
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    /// Reads the next line, returning `None` on EOF.
    pub fn next_line(&mut self) -> io::Result<Option<String>> {
        let mut raw = Vec::with_capacity(256);
        let n = self.inner.read_until(b'\n', &mut raw)?;
        if n == 0 {
            return Ok(None);
        }
        if raw.len() > self.max_line_bytes {
            raw.truncate(self.max_line_bytes);
        }
        // Strip trailing newline
        if raw.last() == Some(&b'\n') {
            raw.pop();
            if raw.last() == Some(&b'\r') {
                raw.pop();
            }
        }
        let line = decode_line(&raw, &self.encoding).into_owned();
        Ok(Some(line))
    }
}
