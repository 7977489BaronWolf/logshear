//! Line sampling for large log files.
//!
//! Supports rate-based (1-in-N) and reservoir sampling strategies.

use rand::Rng;

/// Strategy for sampling lines from a log stream.
#[derive(Debug, Clone)]
pub enum SampleStrategy {
    /// Keep every Nth line (deterministic).
    Rate(usize),
    /// Reservoir sampling: keep at most `size` lines uniformly at random.
    Reservoir(usize),
}

/// A stateful sampler that decides whether to include each line.
pub struct Sampler {
    strategy: SampleStrategy,
    count: usize,
    reservoir: Vec<String>,
    rng: rand::rngs::ThreadRng,
}

impl Sampler {
    pub fn new(strategy: SampleStrategy) -> Self {
        Self {
            strategy,
            count: 0,
            reservoir: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    /// For `Rate` strategy: returns `true` if the line should be kept.
    /// For `Reservoir` strategy: always call `feed` and retrieve results via `into_reservoir`.
    pub fn should_keep(&mut self, line: &str) -> bool {
        self.count += 1;
        match &self.strategy {
            SampleStrategy::Rate(n) => self.count % n == 0,
            SampleStrategy::Reservoir(size) => {
                let size = *size;
                if self.reservoir.len() < size {
                    self.reservoir.push(line.to_string());
                    true
                } else {
                    let j = self.rng.gen_range(0..self.count);
                    if j < size {
                        self.reservoir[j] = line.to_string();
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    /// Consume the sampler and return the reservoir (only meaningful for `Reservoir` strategy).
    pub fn into_reservoir(self) -> Vec<String> {
        self.reservoir
    }

    pub fn lines_seen(&self) -> usize {
        self.count
    }
}
