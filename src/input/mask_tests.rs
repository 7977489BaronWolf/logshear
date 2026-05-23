#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::input::mask::{MaskConfig, MaskRule};
    use crate::input::mask_config::{RawMaskEntry, build_mask_config};

    fn make_config(rules: Vec<(&str, MaskRule)>) -> MaskConfig {
        let fields = rules.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
        MaskConfig::new(fields)
    }

    #[test]
    fn test_prefix_mask_on_record() {
        let cfg = make_config(vec![("token", MaskRule::Prefix(4))]);
        let mut record = HashMap::new();
        record.insert("token".into(), "abcdefgh".into());
        cfg.apply(&mut record);
        assert_eq!(record["token"], "abcd****");
    }

    #[test]
    fn test_suffix_mask_on_record() {
        let cfg = make_config(vec![("card", MaskRule::Suffix(4))]);
        let mut record = HashMap::new();
        record.insert("card".into(), "1234567890123456".into());
        cfg.apply(&mut record);
        assert!(record["card"].ends_with("3456"));
        assert!(record["card"].starts_with('*'));
    }

    #[test]
    fn test_full_mask_on_record() {
        let cfg = make_config(vec![("password", MaskRule::Full("[REDACTED]".into()))]);
        let mut record = HashMap::new();
        record.insert("password".into(), "s3cr3t".into());
        cfg.apply(&mut record);
        assert_eq!(record["password"], "[REDACTED]");
    }

    #[test]
    fn test_prefix_shorter_than_value() {
        let cfg = make_config(vec![("tok", MaskRule::Prefix(100))]);
        let mut record = HashMap::new();
        record.insert("tok".into(), "abc".into());
        cfg.apply(&mut record);
        // nothing to mask — value shorter than prefix
        assert_eq!(record["tok"], "abc");
    }

    #[test]
    fn test_mask_in_line_prefix() {
        let cfg = make_config(vec![("token", MaskRule::Prefix(3))]);
        let line = "level=info token=abcdef msg=ok";
        let result = cfg.apply_to_line(line);
        assert!(result.contains("token=abc***"));
    }

    #[test]
    fn test_mask_in_line_no_field() {
        let cfg = make_config(vec![("secret", MaskRule::Full("***".into()))]);
        let line = "level=info msg=hello";
        let result = cfg.apply_to_line(line);
        assert_eq!(result, line);
    }

    #[test]
    fn test_build_mask_config_prefix() {
        let mut entries = HashMap::new();
        entries.insert("api_key".to_string(), RawMaskEntry::new_prefix(6));
        let cfg = build_mask_config(entries).unwrap();
        let mut record = HashMap::new();
        record.insert("api_key".into(), "ABCDEFGHIJ".into());
        cfg.apply(&mut record);
        assert_eq!(record["api_key"], "ABCDEF****");
    }

    #[test]
    fn test_build_mask_config_invalid_mode() {
        let mut entries = HashMap::new();
        entries.insert("x".to_string(), RawMaskEntry { mode: "unknown".into(), n: None, placeholder: None });
        assert!(build_mask_config(entries).is_err());
    }
}
