//! Match highlighting: annotates spans in a line that matched a query predicate.

use std::ops::Range;

/// A highlighted span within a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Highlight {
    pub range: Range<usize>,
    pub kind: HighlightKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighlightKind {
    /// A literal string match.
    Literal,
    /// A regex match.
    Regex,
    /// A field key match.
    FieldKey,
    /// A field value match.
    FieldValue,
}

/// Collect byte-offset highlight spans from `line` for the given `term`.
pub fn highlight_literal(line: &str, term: &str) -> Vec<Highlight> {
    if term.is_empty() {
        return vec![];
    }
    let mut spans = Vec::new();
    let mut start = 0;
    while let Some(pos) = line[start..].find(term) {
        let abs = start + pos;
        spans.push(Highlight {
            range: abs..abs + term.len(),
            kind: HighlightKind::Literal,
        });
        start = abs + term.len();
    }
    spans
}

/// Render a line with ANSI bold+yellow highlighting for the given spans.
/// Spans must be sorted and non-overlapping.
pub fn render_highlighted(line: &str, spans: &[Highlight]) -> String {
    const BOLD_YELLOW: &str = "\x1b[1;33m";
    const RESET: &str = "\x1b[0m";

    if spans.is_empty() {
        return line.to_owned();
    }

    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len() + spans.len() * 16);
    let mut cursor = 0usize;

    for span in spans {
        if span.range.start > cursor {
            out.push_str(&line[cursor..span.range.start]);
        }
        out.push_str(BOLD_YELLOW);
        out.push_str(&line[span.range.clone()]);
        out.push_str(RESET);
        cursor = span.range.end;
    }

    if cursor < bytes.len() {
        out.push_str(&line[cursor..]);
    }

    out
}
