#[cfg(test)]
mod tests {
    use super::super::glob::{glob_match, GlobExpander};
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn glob_match_pub(pattern: &str, text: &str) -> bool {
        // re-export for test access via the private helper
        // We test through GlobExpander indirectly; also test the inner logic:
        crate::input::glob::glob_match_pub(pattern, text)
    }

    #[test]
    fn test_exact_match() {
        assert!(glob_match_pub("app.log", "app.log"));
        assert!(!glob_match_pub("app.log", "app.txt"));
    }

    #[test]
    fn test_star_wildcard() {
        assert!(glob_match_pub("*.log", "app.log"));
        assert!(glob_match_pub("*.log", "server.log"));
        assert!(!glob_match_pub("*.log", "app.txt"));
        assert!(glob_match_pub("app*", "app.log"));
        assert!(glob_match_pub("app*", "application"));
    }

    #[test]
    fn test_question_wildcard() {
        assert!(glob_match_pub("app.lo?", "app.log"));
        assert!(glob_match_pub("app.lo?", "app.lot"));
        assert!(!glob_match_pub("app.lo?", "app.lo"));
    }

    #[test]
    fn test_combined_wildcards() {
        assert!(glob_match_pub("*.lo?", "server.log"));
        assert!(!glob_match_pub("*.lo?", "server.txt"));
    }

    #[test]
    fn test_expand_plain_path() {
        let dir = TempDir::new().unwrap();
        let file = dir.path().join("test.log");
        fs::write(&file, b"hello").unwrap();

        let expander = GlobExpander::new(dir.path());
        let result = expander.expand("test.log").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], file);
    }

    #[test]
    fn test_expand_plain_path_missing() {
        let dir = TempDir::new().unwrap();
        let expander = GlobExpander::new(dir.path());
        assert!(expander.expand("missing.log").is_err());
    }

    #[test]
    fn test_expand_glob_pattern() {
        let dir = TempDir::new().unwrap();
        for name in &["app.log", "server.log", "app.txt"] {
            fs::write(dir.path().join(name), b"data").unwrap();
        }

        let expander = GlobExpander::new(dir.path());
        let mut result = expander.expand("*.log").unwrap();
        result.sort();
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|p| p.extension().unwrap() == "log"));
    }

    #[test]
    fn test_expand_glob_no_matches() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("app.txt"), b"data").unwrap();

        let expander = GlobExpander::new(dir.path());
        let result = expander.expand("*.log").unwrap();
        assert!(result.is_empty());
    }
}
