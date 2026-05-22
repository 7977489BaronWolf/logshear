use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use std::path::Path;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;

pub enum DecompressedReader {
    Plain(BufReader<File>),
    Gzip(BufReader<GzDecoder<File>>),
    Bzip2(BufReader<BzDecoder<File>>),
}

impl DecompressedReader {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext {
            "gz" => {
                let file = File::open(path)?;
                let decoder = GzDecoder::new(file);
                Ok(DecompressedReader::Gzip(BufReader::new(decoder)))
            }
            "bz2" => {
                let file = File::open(path)?;
                let decoder = BzDecoder::new(file);
                Ok(DecompressedReader::Bzip2(BufReader::new(decoder)))
            }
            _ => {
                let file = File::open(path)?;
                Ok(DecompressedReader::Plain(BufReader::new(file)))
            }
        }
    }

    pub fn is_compressed<P: AsRef<Path>>(path: P) -> bool {
        let ext = path
            .as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        matches!(ext, "gz" | "bz2")
    }
}

impl Read for DecompressedReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            DecompressedReader::Plain(r) => r.read(buf),
            DecompressedReader::Gzip(r) => r.read(buf),
            DecompressedReader::Bzip2(r) => r.read(buf),
        }
    }
}

impl BufRead for DecompressedReader {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            DecompressedReader::Plain(r) => r.fill_buf(),
            DecompressedReader::Gzip(r) => r.fill_buf(),
            DecompressedReader::Bzip2(r) => r.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            DecompressedReader::Plain(r) => r.consume(amt),
            DecompressedReader::Gzip(r) => r.consume(amt),
            DecompressedReader::Bzip2(r) => r.consume(amt),
        }
    }
}
