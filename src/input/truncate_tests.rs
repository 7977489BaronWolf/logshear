#[cfg(test)]
mod tests {
    use super::super::truncate::{truncate_line, TruncateConfig, TruncateIter};

    fn default_cfg() -> TruncateConfig {
        TruncateConfig::default()
    }

    #[test]
    fn short_line_unchanged() {
        let cfg = default_cfg();
        let line = "hello world";
        let result = truncate_line(line, &cfg);
        assert_eq!(result, line);
        // Should be borrowed, not owned.
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
    }

    #[test]
    fn line_exactly_at_limit_unchanged() {
        let cfg = TruncateConfig { max_bytes: 5, suffix: "...".into() };
        let line = "hello";
        let result = truncate_line(line, &cfg);
        assert_eq!(result, "hello");
    }

    #[test]
    fn long_line_truncated_with_suffix() {
        let cfg = TruncateConfig { max_bytes: 10, suffix: "...".into() };
        let line = "abcdefghijklmnopqrstuvwxyz";
        let result = truncate_line(line, &cfg);
        assert_eq!(result, "abcdefg...");
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn truncation_respects_utf8_boundary() {
        // Each '€' is 3 bytes (0xE2 0x82 0xAC).
        let cfg = TruncateConfig { max_bytes: 7, suffix: "!".into() };
        let line = "€€€€"; // 12 bytes total
        let result = truncate_line(line, &cfg);
        // 6 bytes for two '€' chars + 1 byte suffix = 7
        assert_eq!(result, "€€!");
        assert!(result.len() <= 7);
    }

    #[test]
    fn custom_suffix() {
        let cfg = TruncateConfig { max_bytes: 15, suffix: "[cut]".into() };
        let line = "0123456789abcdef";
        let result = truncate_line(line, &cfg);
        assert_eq!(result, "0123456789[cut]");
    }

    #[test]
    fn truncate_iter_applies_to_all_lines() {
        let cfg = TruncateConfig { max_bytes: 5, suffix: "~".into() };
        let lines = vec![
            "hi".to_string(),
            "toolongline".to_string(),
            "ok".to_string(),
        ];
        let result: Vec<String> = TruncateIter::new(lines.into_iter(), cfg).collect();
        assert_eq!(result[0], "hi");
        assert_eq!(result[1], "tool~");
        assert_eq!(result[2], "ok");
    }

    #[test]
    fn suffix_longer_than_max_bytes_produces_empty_prefix() {
        let cfg = TruncateConfig { max_bytes: 3, suffix: "...[truncated]".into() };
        let line = "abcdefgh";
        let result = truncate_line(line, &cfg);
        // target = 3 - 14 = 0 (saturating), so prefix is empty
        assert!(result.ends_with("...[truncated]"));
    }
}
