//! Pipeline stage definitions.
//!
//! Each stage transforms or filters a stream of log lines.

use crate::input::sampler::{SampleStrategy, Sampler};
use crate::query::eval::Evaluator;

/// A single processing stage in the log pipeline.
pub enum Stage {
    /// Filter lines using a compiled query expression.
    Filter(Evaluator),
    /// Sample lines using a given strategy.
    Sample(Sampler),
    /// Limit output to at most N lines.
    Limit(usize),
    /// Skip the first N lines.
    Skip(usize),
}

impl Stage {
    pub fn filter(evaluator: Evaluator) -> Self {
        Stage::Filter(evaluator)
    }

    pub fn rate_sample(n: usize) -> Self {
        Stage::Sample(Sampler::new(SampleStrategy::Rate(n)))
    }

    pub fn reservoir_sample(size: usize) -> Self {
        Stage::Sample(Sampler::new(SampleStrategy::Reservoir(size)))
    }

    pub fn limit(n: usize) -> Self {
        Stage::Limit(n)
    }

    pub fn skip(n: usize) -> Self {
        Stage::Skip(n)
    }

    /// Apply the stage to a single line. Returns `true` if the line should pass through.
    pub fn apply(&mut self, line: &str, emitted: usize, skipped: usize) -> bool {
        match self {
            Stage::Filter(eval) => eval.matches(line),
            Stage::Sample(sampler) => sampler.should_keep(line),
            Stage::Limit(n) => emitted < *n,
            Stage::Skip(n) => skipped >= *n,
        }
    }
}
