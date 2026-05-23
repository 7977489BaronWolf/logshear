#[cfg(test)]
mod tests {
    use super::super::merge::{LineMerger, MergeConfig};

    fn merger(pattern: &str) -> LineMerger {
        LineMerger::new(MergeConfig::new(pattern, " ", 10).unwrap())
    }

    #[test]
    fn single_line_flushed_at_end() {
        let mut m = merger(r"^\s+");
        assert_eq!(m.push("hello".into()), None);
        assert_eq!(m.flush(), Some("hello".into()));
    }

    #[test]
    fn continuation_merged_into_previous() {
        let mut m = merger(r"^\s+");
        assert_eq!(m.push("line one".into()), None);
        assert_eq!(m.push("  continued".into()), None);
        assert_eq!(m.flush(), Some("line one   continued".into()));
    }

    #[test]
    fn non_continuation_flushes_previous() {
        let mut m = merger(r"^\s+");
        assert_eq!(m.push("first".into()), None);
        let out = m.push("second".into());
        assert_eq!(out, Some("first".into()));
        assert_eq!(m.flush(), Some("second".into()));
    }

    #[test]
    fn multiple_continuations() {
        let mut m = merger(r"^\t");
        m.push("start".into());
        m.push("\tcont1".into());
        m.push("\tcont2".into());
        let result = m.flush().unwrap();
        assert_eq!(result, "start \tcont1 \tcont2");
    }

    #[test]
    fn max_lines_respected() {
        let mut m = LineMerger::new(MergeConfig::new(r"^>", " ", 2).unwrap());
        m.push("base".into());
        m.push(">a".into());
        m.push(">b".into());
        // Third continuation exceeds max; it starts a new pending record.
        let flushed = m.push(">c".into());
        assert_eq!(flushed, Some("base >a >b".into()));
        assert_eq!(m.flush(), Some(">c".into()));
    }

    #[test]
    fn flush_on_empty_returns_none() {
        let mut m = merger(r"^\s+");
        assert_eq!(m.flush(), None);
    }

    #[test]
    fn custom_separator() {
        let mut m = LineMerger::new(MergeConfig::new(r"^\|", "\n", 5).unwrap());
        m.push("header".into());
        m.push("|detail".into());
        let result = m.flush().unwrap();
        assert_eq!(result, "header\n|detail");
    }
}
