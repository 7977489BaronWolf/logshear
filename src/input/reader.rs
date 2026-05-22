use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read, Stdin};
use std::path::Path;

pub enum LogSource {
    File(BufReader<File>),
    Stdin(BufReader<Stdin>),
}

impl LogSource {
    pub fn from_path(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(LogSource::File(BufReader::new(file)))
    }

    pub fn from_stdin() -> Self {
        LogSource::Stdin(BufReader::new(io::stdin()))
    }
}

impl Read for LogSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            LogSource::File(r) => r.read(buf),
            LogSource::Stdin(r) => r.read(buf),
        }
    }
}

impl BufRead for LogSource {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            LogSource::File(r) => r.fill_buf(),
            LogSource::Stdin(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            LogSource::File(r) => r.consume(amt),
            LogSource::Stdin(r) => r.consume(amt),
        }
    }
}

pub struct LogReader {
    source: Lines<LogSource>,
    line_number: usize,
}

impl LogReader {
    pub fn new(source: LogSource) -> Self {
        LogReader {
            source: source.lines(),
            line_number: 0,
        }
    }

    pub fn line_number(&self) -> usize {
        self.line_number
    }
}

impl Iterator for LogReader {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.source.next()?;
        self.line_number += 1;
        Some(line)
    }
}
