#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::super::batch::{Batch, BatchCollector, BatchConfig};

    fn fast_config(max_size: usize) -> BatchConfig {
        BatchConfig {
            max_size,
            max_age: Duration::from_secs(60), // long age so size triggers flush
        }
    }

    #[test]
    fn batch_starts_empty() {
        let b = Batch::new();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
    }

    #[test]
    fn batch_push_increments_len() {
        let mut b = Batch::new();
        b.push("hello".to_string());
        b.push("world".to_string());
        assert_eq!(b.len(), 2);
        assert!(!b.is_empty());
    }

    #[test]
    fn batch_age_is_some() {
        let b = Batch::new();
        assert!(b.age().is_some());
    }

    #[test]
    fn collector_no_flush_below_threshold() {
        let mut col = BatchCollector::new(fast_config(4));
        for i in 0..3 {
            let result = col.add(format!("line {}", i));
            assert!(result.is_none(), "should not flush before threshold");
        }
    }

    #[test]
    fn collector_flushes_at_max_size() {
        let mut col = BatchCollector::new(fast_config(3));
        col.add("a".to_string());
        col.add("b".to_string());
        let batch = col.add("c".to_string());
        assert!(batch.is_some());
        let b = batch.unwrap();
        assert_eq!(b.len(), 3);
        assert_eq!(b.lines, vec!["a", "b", "c"]);
    }

    #[test]
    fn collector_resets_after_flush() {
        let mut col = BatchCollector::new(fast_config(2));
        col.add("x".to_string());
        col.add("y".to_string()); // triggers flush
        assert!(col.is_empty());
    }

    #[test]
    fn collector_manual_flush_returns_lines() {
        let mut col = BatchCollector::new(fast_config(100));
        col.add("one".to_string());
        col.add("two".to_string());
        let batch = col.flush();
        assert_eq!(batch.len(), 2);
        assert!(col.is_empty());
    }

    #[test]
    fn collector_flush_empty_batch() {
        let mut col = BatchCollector::new(fast_config(10));
        let batch = col.flush();
        assert!(batch.is_empty());
    }

    #[test]
    fn collector_age_based_flush() {
        let config = BatchConfig {
            max_size: 1000,
            max_age: Duration::from_nanos(1),
        };
        let mut col = BatchCollector::new(config);
        col.add("line1".to_string());
        // Sleep briefly so the batch ages past max_age
        std::thread::sleep(Duration::from_millis(5));
        assert!(col.should_flush());
    }
}
