#[cfg(test)]
mod tests {
    use crate::stats::{Aggregator, StatsCollector, StatsReport};
    use std::thread;
    use std::time::Duration;

    fn make_collector(lines: u64, matched: u64, bytes: u64) -> StatsCollector {
        let mut c = StatsCollector::new();
        c.start();
        for i in 0..lines {
            c.record_line(bytes / lines.max(1), i < matched);
        }
        c
    }

    #[test]
    fn test_basic_recording() {
        let mut c = StatsCollector::new();
        c.start();
        c.record_line(100, true);
        c.record_line(200, false);
        c.record_line(150, true);

        assert_eq!(c.lines_read, 3);
        assert_eq!(c.lines_matched, 2);
        assert_eq!(c.lines_skipped, 1);
        assert_eq!(c.bytes_read, 450);
    }

    #[test]
    fn test_match_rate_zero_lines() {
        let c = StatsCollector::new();
        assert_eq!(c.match_rate(), 0.0);
    }

    #[test]
    fn test_match_rate() {
        let c = make_collector(10, 5, 1000);
        assert!((c.match_rate() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_errors() {
        let mut c = StatsCollector::new();
        c.record_parse_error();
        c.record_parse_error();
        assert_eq!(c.parse_errors, 2);
    }

    #[test]
    fn test_report_display_contains_key_fields() {
        let c = make_collector(1000, 300, 512_000);
        let report = StatsReport::from_collector(&c);
        let output = format!("{}", report);
        assert!(output.contains("1000"));
        assert!(output.contains("300"));
        assert!(output.contains("logshear stats"));
    }

    #[test]
    fn test_aggregator_merge() {
        let c1 = make_collector(500, 100, 50_000);
        let c2 = make_collector(500, 200, 50_000);

        let mut agg = Aggregator::new();
        agg.add(c1);
        agg.add(c2);

        assert_eq!(agg.collector_count(), 2);

        let merged = agg.merge();
        assert_eq!(merged.lines_read, 1000);
        assert_eq!(merged.lines_matched, 300);
        assert_eq!(merged.bytes_read, 100_000);
    }

    #[test]
    fn test_throughput_nonzero_after_sleep() {
        let mut c = StatsCollector::new();
        c.start();
        thread::sleep(Duration::from_millis(10));
        c.record_line(1024, true);
        assert!(c.throughput_lines() > 0.0);
        assert!(c.throughput_bytes() > 0.0);
    }
}
