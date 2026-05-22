use super::StatsCollector;
use std::fmt;

/// A formatted summary of pipeline execution statistics.
#[derive(Debug)]
pub struct StatsReport {
    pub lines_read: u64,
    pub lines_matched: u64,
    pub lines_skipped: u64,
    pub bytes_read: u64,
    pub files_processed: u64,
    pub parse_errors: u64,
    pub elapsed_secs: f64,
    pub match_rate: f64,
    pub throughput_lines: f64,
    pub throughput_bytes: f64,
}

impl StatsReport {
    pub fn from_collector(c: &StatsCollector) -> Self {
        Self {
            lines_read: c.lines_read,
            lines_matched: c.lines_matched,
            lines_skipped: c.lines_skipped,
            bytes_read: c.bytes_read,
            files_processed: c.files_processed,
            parse_errors: c.parse_errors,
            elapsed_secs: c.elapsed().as_secs_f64(),
            match_rate: c.match_rate(),
            throughput_lines: c.throughput_lines(),
            throughput_bytes: c.throughput_bytes(),
        }
    }
}

impl fmt::Display for StatsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- logshear stats ---")?;
        writeln!(f, "  files processed : {}", self.files_processed)?;
        writeln!(f, "  lines read      : {}", self.lines_read)?;
        writeln!(f, "  lines matched   : {} ({:.1}%)", self.lines_matched, self.match_rate)?;
        writeln!(f, "  lines skipped   : {}", self.lines_skipped)?;
        writeln!(f, "  parse errors    : {}", self.parse_errors)?;
        writeln!(f, "  bytes read      : {}", format_bytes(self.bytes_read))?;
        writeln!(f, "  elapsed         : {:.3}s", self.elapsed_secs)?;
        writeln!(f, "  throughput      : {:.0} lines/s  {}/s",
            self.throughput_lines, format_bytes(self.throughput_bytes as u64))?;
        Ok(())
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.2} GiB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MiB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KiB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
