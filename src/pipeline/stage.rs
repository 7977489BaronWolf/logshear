use crate::query::ast::Expr;
use crate::query::eval::{eval_expr, EvalContext};
use crate::output::formatter::{Format, format_record};

/// Result of processing a single log line through the pipeline.
#[derive(Debug)]
pub enum StageResult {
    /// Line passed all filters; contains the (possibly projected) output string.
    Pass(String),
    /// Line was filtered out and should be skipped.
    Skip,
    /// A non-fatal parse warning occurred; line is still emitted.
    Warn(String, String),
}

/// A single processing stage: filter + optional field projection.
#[derive(Debug, Clone)]
pub struct Stage {
    /// Optional boolean filter expression.
    pub filter: Option<Expr>,
    /// Fields to extract/project. Empty means emit the whole line.
    pub fields: Vec<String>,
    /// Output format for this stage.
    pub format: Format,
}

impl Stage {
    pub fn new(filter: Option<Expr>, fields: Vec<String>, format: Format) -> Self {
        Self { filter, fields, format }
    }

    /// Process one log line through this stage.
    pub fn process(&self, line: &str) -> StageResult {
        let ctx = match EvalContext::parse(line) {
            Ok(c) => c,
            Err(e) => return StageResult::Warn(line.to_string(), e.to_string()),
        };

        // Apply filter
        if let Some(expr) = &self.filter {
            match eval_expr(expr, &ctx) {
                Ok(true) => {}
                Ok(false) => return StageResult::Skip,
                Err(e) => return StageResult::Warn(line.to_string(), e.to_string()),
            }
        }

        // Project fields or emit full record
        let output = format_record(&ctx, &self.fields, self.format);
        StageResult::Pass(output)
    }
}
