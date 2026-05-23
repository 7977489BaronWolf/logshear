#[cfg(test)]
mod tests {
    use super::super::field_extractor::extract_fields;
    use super::super::field_filter::FieldFilter;

    fn make_fields(line: &str) -> super::super::field_extractor::ExtractedFields {
        extract_fields(line)
    }

    #[test]
    fn test_passthrough_no_config() {
        let f = FieldFilter::default();
        assert!(f.is_passthrough());
    }

    #[test]
    fn test_include_filter() {
        let f = FieldFilter::new(vec!["level".into(), "msg".into()], vec![]);
        let extracted = make_fields(r#"{"level":"info","msg":"ok","host":"web01"}"#);
        let pairs = f.apply(&extracted);
        let keys: Vec<&str> = pairs.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec!["level", "msg"]);
    }

    #[test]
    fn test_exclude_filter() {
        let f = FieldFilter::new(vec![], vec!["host".into()]);
        let extracted = make_fields(r#"{"level":"info","msg":"ok","host":"web01"}"#);
        let pairs = f.apply(&extracted);
        let keys: Vec<&str> = pairs.iter().map(|(k, _)| *k).collect();
        assert!(!keys.contains(&"host"));
        assert!(keys.contains(&"level"));
    }

    #[test]
    fn test_format_fields_raw_fallback() {
        let f = FieldFilter::default();
        let extracted = make_fields("plain unstructured log");
        let out = f.format_fields(&extracted);
        assert_eq!(out, "plain unstructured log");
    }

    #[test]
    fn test_format_fields_structured() {
        let f = FieldFilter::new(vec!["level".into(), "msg".into()], vec![]);
        let extracted = make_fields(r#"{"level":"warn","msg":"high cpu","host":"srv1"}"#);
        let out = f.format_fields(&extracted);
        assert_eq!(out, "level=warn msg=high cpu");
    }

    #[test]
    fn test_include_preserves_order() {
        let f = FieldFilter::new(vec!["msg".into(), "level".into()], vec![]);
        let extracted = make_fields(r#"{"level":"error","msg":"crash"}"#);
        let pairs = f.apply(&extracted);
        assert_eq!(pairs[0].0, "msg");
        assert_eq!(pairs[1].0, "level");
    }
}
