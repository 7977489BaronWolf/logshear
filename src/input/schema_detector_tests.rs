#[cfg(test)]
mod tests {
    use super::super::schema_detector::SchemaDetector;

    fn json_line(status: u32, msg: &str) -> String {
        format!(r#"{{"status":{},"msg":"{}"}}", status, msg)
    }

    #[test]
    fn test_buffers_until_sample_size() {
        let mut det = SchemaDetector::new(3);
        assert!(det.feed(json_line(200, "a")).is_empty());
        assert!(det.feed(json_line(201, "b")).is_empty());
        assert!(!det.is_ready());
        let out = det.feed(json_line(202, "c"));
        assert_eq!(out.len(), 3);
        assert!(det.is_ready());
    }

    #[test]
    fn test_passes_through_after_schema_ready() {
        let mut det = SchemaDetector::new(2);
        det.feed(json_line(200, "a"));
        det.feed(json_line(201, "b"));
        let out = det.feed(json_line(202, "c"));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0], json_line(202, "c"));
    }

    #[test]
    fn test_flush_finalizes_partial_buffer() {
        let mut det = SchemaDetector::new(10);
        det.feed(json_line(200, "only"));
        assert!(!det.is_ready());
        let out = det.flush();
        assert_eq!(out.len(), 1);
        assert!(det.is_ready());
    }

    #[test]
    fn test_flush_empty_after_full_sample() {
        let mut det = SchemaDetector::new(2);
        det.feed(json_line(200, "a"));
        det.feed(json_line(201, "b"));
        let flushed = det.flush();
        assert!(flushed.is_empty());
    }

    #[test]
    fn test_schema_contains_expected_fields() {
        let mut det = SchemaDetector::new(2);
        det.feed(json_line(200, "hello"));
        det.feed(json_line(404, "not found"));
        let schema = det.schema().expect("schema should be set");
        assert!(schema.get("status").is_some());
        assert!(schema.get("msg").is_some());
    }

    #[test]
    fn test_no_schema_before_any_feed() {
        let det = SchemaDetector::new(5);
        assert!(det.schema().is_none());
        assert!(!det.is_ready());
    }
}
