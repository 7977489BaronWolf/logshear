#[cfg(test)]
mod tests {
    use crate::input::tag_config::TagConfig;

    fn s(v: &str) -> String { v.to_owned() }

    #[test]
    fn parses_single_tag() {
        let cfg = TagConfig::from_args(&[s("env=prod")], false).unwrap();
        assert_eq!(cfg.tags.get("env").map(|v| v.as_str()), Some("prod"));
    }

    #[test]
    fn parses_multiple_tags() {
        let cfg = TagConfig::from_args(&[s("env=prod"), s("region=us-east")], false).unwrap();
        assert_eq!(cfg.tags.len(), 2);
    }

    #[test]
    fn value_may_contain_equals() {
        let cfg = TagConfig::from_args(&[s("token=abc=def")], false).unwrap();
        assert_eq!(cfg.tags.get("token").map(|v| v.as_str()), Some("abc=def"));
    }

    #[test]
    fn empty_key_returns_error() {
        let result = TagConfig::from_args(&[s("=val")], false);
        assert!(result.is_err());
    }

    #[test]
    fn is_empty_when_no_tags_no_source() {
        let cfg = TagConfig::from_args(&[], false).unwrap();
        assert!(cfg.is_empty());
    }

    #[test]
    fn not_empty_with_inject_source() {
        let cfg = TagConfig::from_args(&[], true).unwrap();
        assert!(!cfg.is_empty());
    }

    #[test]
    fn not_empty_with_tags() {
        let cfg = TagConfig::from_args(&[s("k=v")], false).unwrap();
        assert!(!cfg.is_empty());
    }
}
