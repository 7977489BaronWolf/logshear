#[cfg(test)]
mod tests {
    use super::super::formatter::{Formatter, OutputFormat};
    use serde_json::{json, Value};
    use std::collections::HashMap;

    fn fields(pairs: &[(&str, Value)]) -> HashMap<String, Value> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
    }

    #[test]
    fn raw_format_returns_original_line() {
        let f = Formatter::new(OutputFormat::Raw, false);
        let result = f.format_line("hello world", &HashMap::new());
        assert_eq!(result, Some("hello world".to_string()));
    }

    #[test]
    fn json_format_with_fields() {
        let f = Formatter::new(OutputFormat::Json, false);
        let flds = fields(&[("level", json!("error")), ("msg", json!("oops"))]);
        let result = f.format_line("", &flds).unwrap();
        let parsed: Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed["level"], json!("error"));
    }

    #[test]
    fn json_format_empty_fields_returns_raw() {
        let f = Formatter::new(OutputFormat::Json, false);
        let result = f.format_line("raw line", &HashMap::new());
        assert_eq!(result, Some("raw line".to_string()));
    }

    #[test]
    fn fields_format_extracts_columns() {
        let f = Formatter::new(OutputFormat::Fields(vec!["level".into(), "msg".into()]), false);
        let flds = fields(&[("level", json!("warn")), ("msg", json!("watch out")), ("ts", json!(123))]);
        let result = f.format_line("", &flds).unwrap();
        assert_eq!(result, "warn\twatch out");
    }

    #[test]
    fn fields_format_missing_key_skipped() {
        let f = Formatter::new(OutputFormat::Fields(vec!["level".into(), "missing".into()]), false);
        let flds = fields(&[("level", json!("info"))]);
        let result = f.format_line("", &flds).unwrap();
        assert_eq!(result, "info");
    }

    #[test]
    fn highlight_wraps_term_with_ansi() {
        let f = Formatter::new(OutputFormat::Raw, true);
        let out = f.highlight("error occurred", "error");
        assert!(out.contains("\x1b[1;33merror\x1b[0m"));
    }

    #[test]
    fn highlight_no_color_returns_unchanged() {
        let f = Formatter::new(OutputFormat::Raw, false);
        let out = f.highlight("error occurred", "error");
        assert_eq!(out, "error occurred");
    }
}
