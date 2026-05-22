#[cfg(test)]
mod tests {
    use super::super::highlight::*;

    #[test]
    fn test_no_match_returns_empty() {
        let spans = highlight_literal("hello world", "xyz");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_single_match() {
        let spans = highlight_literal("hello world", "world");
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].range, 6..11);
        assert_eq!(spans[0].kind, HighlightKind::Literal);
    }

    #[test]
    fn test_multiple_matches() {
        let spans = highlight_literal("abcabc", "abc");
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].range, 0..3);
        assert_eq!(spans[1].range, 3..6);
    }

    #[test]
    fn test_empty_term_returns_empty() {
        let spans = highlight_literal("anything", "");
        assert!(spans.is_empty());
    }

    #[test]
    fn test_render_no_spans() {
        let result = render_highlighted("plain text", &[]);
        assert_eq!(result, "plain text");
    }

    #[test]
    fn test_render_single_span() {
        let spans = highlight_literal("hello world", "world");
        let result = render_highlighted("hello world", &spans);
        assert!(result.contains("\x1b[1;33m"));
        assert!(result.contains("world"));
        assert!(result.contains("\x1b[0m"));
        assert!(result.starts_with("hello "));
    }

    #[test]
    fn test_render_multiple_spans() {
        let line = "foo bar foo";
        let spans = highlight_literal(line, "foo");
        let result = render_highlighted(line, &spans);
        // Both occurrences should be wrapped
        assert_eq!(result.matches("\x1b[1;33m").count(), 2);
        assert_eq!(result.matches("\x1b[0m").count(), 2);
    }

    #[test]
    fn test_render_match_at_start() {
        let spans = highlight_literal("error: something", "error");
        let result = render_highlighted("error: something", &spans);
        assert!(result.starts_with("\x1b[1;33m"));
    }

    #[test]
    fn test_render_match_at_end() {
        let spans = highlight_literal("status: ok", "ok");
        let result = render_highlighted("status: ok", &spans);
        assert!(result.ends_with("\x1b[0m"));
    }
}
