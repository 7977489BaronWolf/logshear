#[cfg(test)]
mod tests {
    use super::super::schema::{FieldType, Schema};

    #[test]
    fn test_infer_integer_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[r#"{"status": 200}"#]);
        let f = schema.get("status").expect("field missing");
        assert_eq!(f.inferred_type, FieldType::Integer);
        assert!(!f.nullable);
    }

    #[test]
    fn test_infer_boolean_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[r#"{"ok": true}"#]);
        let f = schema.get("ok").unwrap();
        assert_eq!(f.inferred_type, FieldType::Boolean);
    }

    #[test]
    fn test_infer_float_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[r#"{"latency": 1.23}"#]);
        let f = schema.get("latency").unwrap();
        assert_eq!(f.inferred_type, FieldType::Float);
    }

    #[test]
    fn test_infer_timestamp_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[r#"{"ts": "2024-01-15T08:30:00Z"}"#]);
        let f = schema.get("ts").unwrap();
        assert_eq!(f.inferred_type, FieldType::Timestamp);
    }

    #[test]
    fn test_infer_string_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[r#"{"msg": "hello world"}"#]);
        let f = schema.get("msg").unwrap();
        assert_eq!(f.inferred_type, FieldType::String);
    }

    #[test]
    fn test_nullable_field() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[
            r#"{"user": "alice"}"#,
            r#"{"user": null}"#,
        ]);
        let f = schema.get("user").unwrap();
        assert!(f.nullable);
        assert_eq!(f.sample_count, 2);
    }

    #[test]
    fn test_type_conflict_falls_back_to_string() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[
            r#"{"val": 42}"#,
            r#"{"val": "text"}"#,
        ]);
        let f = schema.get("val").unwrap();
        assert_eq!(f.inferred_type, FieldType::String);
    }

    #[test]
    fn test_non_json_lines_ignored() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&["plain text line", "another plain line"]);
        assert!(schema.fields.is_empty());
    }

    #[test]
    fn test_sample_count_accumulates() {
        let mut schema = Schema::new();
        schema.infer_from_lines(&[
            r#"{"level": "info"}"#,
            r#"{"level": "warn"}"#,
            r#"{"level": "error"}"#,
        ]);
        let f = schema.get("level").unwrap();
        assert_eq!(f.sample_count, 3);
    }
}
