use std::io::{BufRead, Write};
use crate::pipeline::stage::{Stage, StageResult};

/// Statistics collected during a pipeline run.
#[derive(Debug, Default)]
pub struct RunStats {
    pub lines_read: u64,
    pub lines_passed: u64,
    pub lines_skipped: u64,
    pub lines_warned: u64,
}

/// Orchestrates reading input, running stages, and writing output.
pub struct Pipeline {
    stages: Vec<Stage>,
    warn_to_stderr: bool,
}

impl Pipeline {
    pub fn new(stages: Vec<Stage>, warn_to_stderr: bool) -> Self {
        Self { stages, warn_to_stderr }
    }

    /// Run the pipeline: read lines from `reader`, write results to `writer`.
    pub fn run<R: BufRead, W: Write>(
        &self,
        reader: R,
        mut writer: W,
    ) -> std::io::Result<RunStats> {
        let mut stats = RunStats::default();

        for raw in reader.lines() {
            let line = raw?;
            stats.lines_read += 1;

            let mut current = line.as_str().to_string();
            let mut passed = true;

            for stage in &self.stages {
                match stage.process(&current) {
                    StageResult::Pass(out) => {
                        current = out;
                    }
                    StageResult::Skip => {
                        passed = false;
                        stats.lines_skipped += 1;
                        break;
                    }
                    StageResult::Warn(orig, msg) => {
                        stats.lines_warned += 1;
                        if self.warn_to_stderr {
                            eprintln!("[logshear warn] {}: {}", msg, orig);
                        }
                        current = orig;
                    }
                }
            }

            if passed {
                writeln!(writer, "{}", current)?;
                stats.lines_passed += 1;
            }
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod runner_tests {
    use super::*;
    use crate::pipeline::stage::Stage;
    use crate::output::formatter::Format;
    use std::io::Cursor;

    fn make_pipeline(filter: Option<crate::query::ast::Expr>, fields: Vec<String>) -> Pipeline {
        let stage = Stage::new(filter, fields, Format::Plain);
        Pipeline::new(vec![stage], false)
    }

    #[test]
    fn passthrough_no_filter() {
        let input = Cursor::new("hello world\nfoo bar\n");
        let mut output = Vec::new();
        let p = make_pipeline(None, vec![]);
        let stats = p.run(input, &mut output).unwrap();
        assert_eq!(stats.lines_read, 2);
        assert_eq!(stats.lines_passed, 2);
        assert_eq!(stats.lines_skipped, 0);
    }
}
