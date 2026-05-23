#[cfg(test)]
mod tests {
    use super::super::field_extractor::*;

    #[test]
    fn test_extract_json_fields() {
        let line = r#"{"level":"info","msg":"started","port":8080}"#;
        let extracted = extract_fields(line);
        assert_eq!(extracted.get("level"), Some("info"));
        assert_eq!(extracted.get("msg"), Some("started"));
        assert_eq!(extracted.get("port"), Some("8080"));
    }

    #[test]
    fn test_extract_kv_fields() {
        let line = "ts=2024-01-01 level=warn msg=\"disk full\" host=web01";
        let extracted = extract_fields(line);
        assert_eq!(extracted.get("level"), Some("warn"));
        assert_eq!(extracted.get("msg"), Some("disk full"));
        assert_eq!(extracted.get("host"), Some("web01"));
    }

    #[test]
    fn test_extract_unstructured_falls_back_empty() {
        let line = "plain log line with no structure";
        let extracted = extract_fields(line);
        assert!(extracted.fields.is_empty());
        assert_eq!(extracted.raw, line);
    }

    #[test]
    fn test_extract_json_nested_value_stringified() {
        let line = r#"{"meta":{"k":1},"level":"debug"}"#;
        let extracted = extract_fields(line);
        assert_eq!(extracted.get("level"), Some("debug"));
        // nested objects are stringified
        assert!(extracted.get("meta").is_some());
    }

    #[test]
    fn test_get_missing_field() {
        let line = r#"{"level":"info"}"#;
        let extracted = extract_fields(line);
        assert_eq!(extracted.get("nonexistent"), None);
    }

    #[test]
    fn test_kv_simple_values() {
        let line = "status=200 method=GET path=/api/v1";
        let extracted = extract_fields(line);
        assert_eq!(extracted.get("status"), Some("200"));
        assert_eq!(extracted.get("method"), Some("GET"));
        assert_eq!(extracted.get("path"), Some("/api/v1"));
    }

    #[test]
    fn test_raw_preserved() {
        let line = "some raw log line";
        let extracted = extract_fields(line);
        assert_eq!(extracted.raw, line);
    }
}
