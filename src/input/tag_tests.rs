#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::input::tag::Tagger;

    fn tagger(pairs: &[(&str, &str)], inject_source: bool) -> Tagger {
        let map = pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        Tagger::new(map, inject_source)
    }

    #[test]
    fn non_json_line_unchanged() {
        let t = tagger(&[("env", "prod")], false);
        let result = t.apply("plain text log", None);
        assert_eq!(result, "plain text log");
    }

    #[test]
    fn injects_static_tag() {
        let t = tagger(&[("env", "prod")], false);
        let result = t.apply(r#"{"msg":"hello"}", None);
        assert!(result.contains(r#""env":"prod""));
        assert!(result.starts_with('{'));
        assert!(result.ends_with('}'));
    }

    #[test]
    fn injects_source_tag() {
        let t = tagger(&[], true);
        let result = t.apply(r#"{"msg":"hello"}", Some("app.log"));
        assert!(result.contains(r#""_source":"app.log""));
    }

    #[test]
    fn injects_both_tags() {
        let t = tagger(&[("region", "us-east")], true);
        let result = t.apply(r#"{"level":"info"}", Some("svc.log"));
        assert!(result.contains("region"));
        assert!(result.contains("_source"));
    }

    #[test]
    fn no_tags_returns_original() {
        let t = tagger(&[], false);
        let original = r#"{"msg":"hi"}"#;
        assert_eq!(t.apply(original, None), original);
    }

    #[test]
    fn has_tags_false_when_empty() {
        let t = tagger(&[], false);
        assert!(!t.has_tags());
    }

    #[test]
    fn has_tags_true_with_static() {
        let t = tagger(&[("k", "v")], false);
        assert!(t.has_tags());
    }

    #[test]
    fn has_tags_true_with_source() {
        let t = tagger(&[], true);
        assert!(t.has_tags());
    }
}
