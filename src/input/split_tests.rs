#[cfg(test)]
mod tests {
    use super::super::split::{split_line, SplitConfig};

    fn default_cfg() -> SplitConfig {
        SplitConfig::default()
    }

    #[test]
    fn splits_on_space() {
        let cfg = default_cfg();
        let tokens = split_line("hello world foo", &cfg);
        assert_eq!(tokens, vec!["hello", "world", "foo"]);
    }

    #[test]
    fn skips_empty_tokens_by_default() {
        let cfg = default_cfg();
        let tokens = split_line("a  b   c", &cfg);
        // single-space delimiter: consecutive spaces create empty tokens that are skipped
        assert!(!tokens.contains(&std::borrow::Cow::Borrowed("")));
    }

    #[test]
    fn keeps_empty_tokens_when_disabled() {
        let cfg = SplitConfig {
            skip_empty: false,
            trim: false,
            ..default_cfg()
        };
        let tokens = split_line("a  b", &cfg);
        assert!(tokens.iter().any(|t| t.as_ref() == ""));
    }

    #[test]
    fn custom_delimiter() {
        let cfg = SplitConfig {
            delimiter: "|".to_string(),
            ..default_cfg()
        };
        let tokens = split_line("ts=123|level=INFO|msg=ok", &cfg);
        assert_eq!(tokens, vec!["ts=123", "level=INFO", "msg=ok"]);
    }

    #[test]
    fn multi_char_delimiter() {
        let cfg = SplitConfig {
            delimiter: ", ".to_string(),
            ..default_cfg()
        };
        let tokens = split_line("one, two, three", &cfg);
        assert_eq!(tokens, vec!["one", "two", "three"]);
    }

    #[test]
    fn max_tokens_limits_split() {
        let cfg = SplitConfig {
            max_tokens: 2,
            ..default_cfg()
        };
        let tokens = split_line("a b c d", &cfg);
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], "a");
        assert_eq!(tokens[1], "b c d");
    }

    #[test]
    fn max_tokens_one_returns_whole_line() {
        let cfg = SplitConfig {
            max_tokens: 1,
            ..default_cfg()
        };
        let tokens = split_line("  hello world  ", &cfg);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "hello world");
    }

    #[test]
    fn empty_line_returns_empty_vec() {
        let cfg = default_cfg();
        let tokens = split_line("", &cfg);
        assert!(tokens.is_empty());
    }

    #[test]
    fn trims_whitespace_from_tokens() {
        let cfg = SplitConfig {
            delimiter: "|".to_string(),
            trim: true,
            skip_empty: true,
            max_tokens: 0,
        };
        let tokens = split_line(" a | b | c ", &cfg);
        assert_eq!(tokens, vec!["a", "b", "c"]);
    }
}
