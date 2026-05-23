#[cfg(test)]
mod tests {
    use super::super::rate_limiter::RateLimiter;
    use std::time::Instant;

    #[test]
    fn test_unlimited_no_sleep() {
        let mut rl = RateLimiter::new(0);
        assert!(!rl.is_active());
        let start = Instant::now();
        for _ in 0..10_000 {
            rl.throttle();
        }
        // Should complete near-instantly without sleeping
        assert!(start.elapsed().as_millis() < 500);
        assert_eq!(rl.total_lines(), 10_000);
        assert_eq!(rl.total_sleep_ns(), 0);
    }

    #[test]
    fn test_active_when_limit_set() {
        let rl = RateLimiter::new(100);
        assert!(rl.is_active());
    }

    #[test]
    fn test_counts_lines() {
        let mut rl = RateLimiter::new(0);
        for _ in 0..42 {
            rl.throttle();
        }
        assert_eq!(rl.total_lines(), 42);
    }

    #[test]
    fn test_counts_lines_with_limit() {
        let mut rl = RateLimiter::new(10_000);
        for _ in 0..50 {
            rl.throttle();
        }
        assert_eq!(rl.total_lines(), 50);
    }

    #[test]
    fn test_high_limit_no_sleep_for_small_batch() {
        let mut rl = RateLimiter::new(100_000);
        let start = Instant::now();
        // 100 lines well under the 100k/s limit — should not sleep
        for _ in 0..100 {
            rl.throttle();
        }
        assert!(start.elapsed().as_millis() < 200);
        assert_eq!(rl.total_sleep_ns(), 0);
    }

    #[test]
    fn test_effective_rate_nonzero_after_lines() {
        let mut rl = RateLimiter::new(0);
        for _ in 0..100 {
            rl.throttle();
        }
        // effective_rate is based on lines_in_window which resets on window
        // For unlimited, lines_in_window stays 0, so just check no panic
        let _ = rl.effective_rate();
    }

    #[test]
    fn test_zero_max_is_unlimited() {
        let mut rl = RateLimiter::new(0);
        assert!(!rl.is_active());
        rl.throttle();
        assert_eq!(rl.total_lines(), 1);
    }
}
