#[cfg(test)]
mod tests {
    use super::super::deduplicate::Deduplicator;

    #[test]
    fn test_unique_lines_pass_through() {
        let mut dedup = Deduplicator::new(100);
        assert!(dedup.is_unique("line one"));
        assert!(dedup.is_unique("line two"));
        assert!(dedup.is_unique("line three"));
        assert_eq!(dedup.duplicates_removed(), 0);
    }

    #[test]
    fn test_duplicate_line_rejected() {
        let mut dedup = Deduplicator::new(100);
        assert!(dedup.is_unique("repeated line"));
        assert!(!dedup.is_unique("repeated line"));
        assert!(!dedup.is_unique("repeated line"));
        assert_eq!(dedup.duplicates_removed(), 2);
    }

    #[test]
    fn test_window_eviction_allows_reentry() {
        let mut dedup = Deduplicator::new(3);
        assert!(dedup.is_unique("a"));
        assert!(dedup.is_unique("b"));
        assert!(dedup.is_unique("c"));
        // Window is full; adding "d" evicts "a"
        assert!(dedup.is_unique("d"));
        // "a" should now be allowed again
        assert!(dedup.is_unique("a"));
    }

    #[test]
    fn test_window_size_one() {
        let mut dedup = Deduplicator::new(1);
        assert!(dedup.is_unique("x"));
        assert!(!dedup.is_unique("x"));
        assert!(dedup.is_unique("y"));
        // "x" evicted by "y"
        assert!(dedup.is_unique("x"));
    }

    #[test]
    fn test_reset_clears_state() {
        let mut dedup = Deduplicator::new(100);
        assert!(dedup.is_unique("hello"));
        assert!(!dedup.is_unique("hello"));
        dedup.reset();
        assert_eq!(dedup.duplicates_removed(), 0);
        assert!(dedup.is_unique("hello"));
    }

    #[test]
    fn test_empty_string_deduplication() {
        let mut dedup = Deduplicator::new(10);
        assert!(dedup.is_unique(""));
        assert!(!dedup.is_unique(""));
        assert_eq!(dedup.duplicates_removed(), 1);
    }

    #[test]
    fn test_default_window_size() {
        let dedup = Deduplicator::default();
        assert_eq!(dedup.duplicates_removed(), 0);
    }
}
