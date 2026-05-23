use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Deduplicates log lines within a sliding window of recent entries.
pub struct Deduplicator {
    seen: HashSet<u64>,
    window_size: usize,
    order: std::collections::VecDeque<u64>,
    duplicates_removed: u64,
}

impl Deduplicator {
    pub fn new(window_size: usize) -> Self {
        Self {
            seen: HashSet::new(),
            window_size,
            order: std::collections::VecDeque::new(),
            duplicates_removed: 0,
        }
    }

    /// Returns `true` if the line is unique (not a duplicate).
    pub fn is_unique(&mut self, line: &str) -> bool {
        let hash = hash_line(line);
        if self.seen.contains(&hash) {
            self.duplicates_removed += 1;
            return false;
        }
        self.seen.insert(hash);
        self.order.push_back(hash);
        if self.order.len() > self.window_size {
            if let Some(evicted) = self.order.pop_front() {
                self.seen.remove(&evicted);
            }
        }
        true
    }

    pub fn duplicates_removed(&self) -> u64 {
        self.duplicates_removed
    }

    pub fn reset(&mut self) {
        self.seen.clear();
        self.order.clear();
        self.duplicates_removed = 0;
    }
}

fn hash_line(line: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    line.hash(&mut hasher);
    hasher.finish()
}

impl Default for Deduplicator {
    fn default() -> Self {
        Self::new(1000)
    }
}
