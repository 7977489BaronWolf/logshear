#[cfg(test)]
mod tests {
    use super::super::decompressor::DecompressedReader;
    use std::io::{BufRead, Write};
    use std::path::Path;
    use tempfile::NamedTempFile;
    use flate2::write::GzEncoder;
    use flate2::Compression;

    #[test]
    fn test_is_compressed_gz() {
        assert!(DecompressedReader::is_compressed("logs/app.log.gz"));
    }

    #[test]
    fn test_is_compressed_bz2() {
        assert!(DecompressedReader::is_compressed("logs/app.log.bz2"));
    }

    #[test]
    fn test_is_compressed_plain() {
        assert!(!DecompressedReader::is_compressed("logs/app.log"));
        assert!(!DecompressedReader::is_compressed("logs/app.txt"));
    }

    #[test]
    fn test_open_plain_file() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "hello plain").unwrap();
        writeln!(tmp, "world").unwrap();

        let mut reader = DecompressedReader::open(tmp.path()).unwrap();
        let mut lines = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            lines.push(line.trim_end().to_string());
            line.clear();
        }
        assert_eq!(lines, vec!["hello plain", "world"]);
    }

    #[test]
    fn test_open_gzip_file() {
        let tmp = NamedTempFile::with_suffix(".gz").unwrap();
        {
            let file = std::fs::File::create(tmp.path()).unwrap();
            let mut enc = GzEncoder::new(file, Compression::default());
            enc.write_all(b"compressed line one\ncompressed line two\n").unwrap();
            enc.finish().unwrap();
        }

        let mut reader = DecompressedReader::open(tmp.path()).unwrap();
        let mut lines = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line).unwrap() > 0 {
            lines.push(line.trim_end().to_string());
            line.clear();
        }
        assert_eq!(lines, vec!["compressed line one", "compressed line two"]);
    }

    #[test]
    fn test_open_nonexistent_file() {
        let result = DecompressedReader::open("/nonexistent/path/file.log");
        assert!(result.is_err());
    }
}
