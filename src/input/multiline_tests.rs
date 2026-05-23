#[cfg(test)]
mod tests {
    use super::super::multiline::{MultilineAssembler, MultilineConfig};

    fn assembler(pattern: &str) -> MultilineAssembler {
        MultilineAssembler::new(MultilineConfig::new(pattern, 64).unwrap())
    }

    #[test]
    fn single_line_records_pass_through() {
        let mut a = assembler(r"^\d{4}-");
        let lines = vec![
            "2024-01-01 INFO hello".to_string(),
            "2024-01-01 INFO world".to_string(),
        ];
        let records = a.process_lines(lines);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0], "2024-01-01 INFO hello");
        assert_eq!(records[1], "2024-01-01 INFO world");
    }

    #[test]
    fn continuation_lines_are_joined() {
        let mut a = assembler(r"^\d{4}-");
        let lines = vec![
            "2024-01-01 ERROR oops".to_string(),
            "  at foo.rs:10".to_string(),
            "  at bar.rs:20".to_string(),
            "2024-01-01 INFO done".to_string(),
        ];
        let records = a.process_lines(lines);
        assert_eq!(records.len(), 2);
        assert!(records[0].contains("at foo.rs:10"));
        assert!(records[0].contains("at bar.rs:20"));
        assert_eq!(records[1], "2024-01-01 INFO done");
    }

    #[test]
    fn finish_flushes_remaining_buffer() {
        let mut a = assembler(r"^\d{4}-");
        a.push("2024-01-01 WARN partial".to_string());
        a.push("  continuation".to_string());
        let record = a.finish().expect("should flush");
        assert!(record.contains("continuation"));
        assert!(a.finish().is_none(), "double finish should be empty");
    }

    #[test]
    fn max_lines_triggers_flush() {
        let config = MultilineConfig::new(r"^START", 3).unwrap();
        let mut a = MultilineAssembler::new(config);
        // None of these lines match the start pattern, so they accumulate.
        let r1 = a.push("line1".to_string());
        let r2 = a.push("line2".to_string());
        let r3 = a.push("line3".to_string()); // triggers max_lines flush
        assert!(r1.is_none());
        assert!(r2.is_none());
        assert!(r3.is_some());
        let flushed = r3.unwrap();
        assert!(flushed.contains("line1"));
        assert!(flushed.contains("line3"));
    }

    #[test]
    fn empty_input_produces_no_records() {
        let mut a = assembler(r"^\d{4}-");
        let records = a.process_lines(vec![]);
        assert!(records.is_empty());
    }

    #[test]
    fn invalid_regex_returns_error() {
        let result = MultilineConfig::new(r"[invalid", 10);
        assert!(result.is_err());
    }
}
