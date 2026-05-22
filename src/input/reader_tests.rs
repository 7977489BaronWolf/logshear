#[cfg(test)]
mod tests {
    use super::super::reader::{LogReader, LogSource};
    use std::io::{self, BufRead, Cursor, Read};
    use std::path::Path;

    fn make_reader_from_str(input: &str) -> LogReader {
        // Wrap a Cursor as a File-like source via a helper shim
        // We test LogReader logic using a temporary file approach
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tmp = NamedTempFile::new().expect("temp file");
        tmp.write_all(input.as_bytes()).expect("write");
        let path = tmp.path().to_owned();
        // Keep tmp alive by leaking it for the duration of the test
        std::mem::forget(tmp);
        let source = LogSource::from_path(&path).expect("open");
        LogReader::new(source)
    }

    #[test]
    fn test_reads_all_lines() {
        let input = "line one\nline two\nline three\n";
        let reader = make_reader_from_str(input);
        let lines: Vec<String> = reader.map(|l| l.unwrap()).collect();
        assert_eq!(lines, vec!["line one", "line two", "line three"]);
    }

    #[test]
    fn test_line_number_increments() {
        let input = "alpha\nbeta\ngamma\n";
        let mut reader = make_reader_from_str(input);
        assert_eq!(reader.line_number(), 0);
        reader.next();
        assert_eq!(reader.line_number(), 1);
        reader.next();
        assert_eq!(reader.line_number(), 2);
        reader.next();
        assert_eq!(reader.line_number(), 3);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let reader = make_reader_from_str(input);
        let lines: Vec<String> = reader.map(|l| l.unwrap()).collect();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_single_line_no_newline() {
        let input = "no newline at end";
        let reader = make_reader_from_str(input);
        let lines: Vec<String> = reader.map(|l| l.unwrap()).collect();
        assert_eq!(lines, vec!["no newline at end"]);
    }

    #[test]
    fn test_from_path_missing_file() {
        let result = LogSource::from_path(Path::new("/nonexistent/path/to/file.log"));
        assert!(result.is_err());
    }
}
