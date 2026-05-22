#[cfg(test)]
mod tests {
    use super::super::line_counter::LineCounter;

    #[test]
    fn test_new_counter_is_at_start() {
        let c = LineCounter::new();
        assert!(c.is_at_start());
        assert_eq!(c.line, 0);
        assert_eq!(c.byte_offset, 0);
    }

    #[test]
    fn test_advance_increments_line_and_offset() {
        let mut c = LineCounter::new();
        c.advance(10);
        assert_eq!(c.line, 1);
        assert_eq!(c.byte_offset, 10);
        c.advance(5);
        assert_eq!(c.line, 2);
        assert_eq!(c.byte_offset, 15);
    }

    #[test]
    fn test_position_label_without_source() {
        let mut c = LineCounter::new();
        c.advance(8);
        assert_eq!(c.position_label(), "line 1");
    }

    #[test]
    fn test_position_label_with_source() {
        let mut c = LineCounter::with_source("access.log");
        c.advance(20);
        c.advance(15);
        assert_eq!(c.position_label(), "access.log:2");
    }

    #[test]
    fn test_display_matches_position_label() {
        let mut c = LineCounter::with_source("app.log");
        c.advance(30);
        assert_eq!(format!("{}", c), c.position_label());
    }

    #[test]
    fn test_reset_clears_counters() {
        let mut c = LineCounter::with_source("old.log");
        c.advance(100);
        c.advance(50);
        c.reset(Some("new.log".to_string()));
        assert_eq!(c.line, 0);
        assert_eq!(c.byte_offset, 0);
        assert_eq!(c.source.as_deref(), Some("new.log"));
    }

    #[test]
    fn test_reset_without_source_keeps_existing() {
        let mut c = LineCounter::with_source("keep.log");
        c.advance(10);
        c.reset(None);
        assert_eq!(c.line, 0);
        assert_eq!(c.source.as_deref(), Some("keep.log"));
    }

    #[test]
    fn test_is_at_start_false_after_advance() {
        let mut c = LineCounter::new();
        c.advance(1);
        assert!(!c.is_at_start());
    }
}
