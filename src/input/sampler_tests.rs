#[cfg(test)]
mod tests {
    use super::super::sampler::{SampleStrategy, Sampler};

    #[test]
    fn rate_sampling_keeps_every_nth() {
        let mut s = Sampler::new(SampleStrategy::Rate(3));
        let lines = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        let kept: Vec<&str> = lines
            .iter()
            .filter(|&&l| s.should_keep(l))
            .copied()
            .collect();
        // lines 3, 6, 9 → indices 2, 5, 8
        assert_eq!(kept, vec!["c", "f", "i"]);
        assert_eq!(s.lines_seen(), 9);
    }

    #[test]
    fn rate_sampling_rate_one_keeps_all() {
        let mut s = Sampler::new(SampleStrategy::Rate(1));
        let lines = ["x", "y", "z"];
        let kept: Vec<&str> = lines
            .iter()
            .filter(|&&l| s.should_keep(l))
            .copied()
            .collect();
        assert_eq!(kept, vec!["x", "y", "z"]);
    }

    #[test]
    fn reservoir_sampling_keeps_at_most_size() {
        let mut s = Sampler::new(SampleStrategy::Reservoir(5));
        for i in 0..100 {
            s.should_keep(&format!("line {}", i));
        }
        assert_eq!(s.lines_seen(), 100);
        let res = s.into_reservoir();
        assert_eq!(res.len(), 5);
    }

    #[test]
    fn reservoir_sampling_fewer_lines_than_size() {
        let mut s = Sampler::new(SampleStrategy::Reservoir(20));
        for i in 0..7 {
            s.should_keep(&format!("line {}", i));
        }
        let res = s.into_reservoir();
        assert_eq!(res.len(), 7);
    }

    #[test]
    fn reservoir_all_lines_represented_statistically() {
        // Run many trials; every line index should appear at least once.
        let trials = 500;
        let n_lines = 10;
        let reservoir_size = 5;
        let mut seen = vec![0usize; n_lines];
        for _ in 0..trials {
            let mut s = Sampler::new(SampleStrategy::Reservoir(reservoir_size));
            for i in 0..n_lines {
                s.should_keep(&format!("{}", i));
            }
            for entry in s.into_reservoir() {
                let idx: usize = entry.parse().unwrap();
                seen[idx] += 1;
            }
        }
        for (i, &count) in seen.iter().enumerate() {
            assert!(count > 0, "line {} never appeared in reservoir", i);
        }
    }
}
