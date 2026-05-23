#[cfg(test)]
mod tests {
    use super::super::timeout::{TimeoutConfig, TimeoutReader};
    use std::io::{BufReader, Cursor};
    use std::time::Duration;

    fn make_reader(data: &str, secs: u64) -> TimeoutReader<BufReader<Cursor<Vec<u8>>>> {
        let cursor = Cursor::new(data.as_bytes().to_vec());
        TimeoutReader::new(BufReader::new(cursor), Duration::from_secs(secs))
    }

    #[test]
    fn test_read_line_within_timeout() {
        let mut reader = make_reader("hello\nworld\n", 5);
        let mut buf = String::new();
        let result = reader.read_line_timeout(&mut buf).unwrap();
        assert!(result.is_some());
        assert_eq!(buf, "hello\n");
    }

    #[test]
    fn test_read_all_lines() {
        let mut reader = make_reader("line1\nline2\nline3\n", 10);
        let mut lines = vec![];
        loop {
            let mut buf = String::new();
            match reader.read_line_timeout(&mut buf).unwrap() {
                Some(0) | None => break,
                Some(_) => lines.push(buf.trim_end().to_string()),
            }
        }
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_not_timed_out_immediately() {
        let reader = make_reader("data\n", 10);
        assert!(!reader.is_timed_out());
    }

    #[test]
    fn test_elapsed_increases() {
        let reader = make_reader("", 5);
        let e1 = reader.elapsed();
        std::thread::sleep(Duration::from_millis(10));
        let e2 = reader.elapsed();
        assert!(e2 >= e1);
    }

    #[test]
    fn test_reset_timer() {
        let mut reader = make_reader("", 5);
        std::thread::sleep(Duration::from_millis(5));
        reader.reset_timer();
        assert!(reader.elapsed() < Duration::from_millis(5));
    }

    #[test]
    fn test_timeout_config_default() {
        let cfg = TimeoutConfig::default();
        assert_eq!(cfg.line_timeout, Duration::from_secs(5));
        assert_eq!(cfg.idle_timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_config_custom() {
        let cfg = TimeoutConfig {
            line_timeout: Duration::from_millis(200),
            idle_timeout: Duration::from_secs(10),
        };
        assert_eq!(cfg.line_timeout, Duration::from_millis(200));
    }

    #[test]
    fn test_into_inner() {
        let reader = make_reader("abc\n", 5);
        let inner = reader.into_inner();
        drop(inner);
    }
}
