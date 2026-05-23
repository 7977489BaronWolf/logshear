//! Line splitting / field tokenization for structured log lines.
//!
//! Splits a raw log line into tokens by a configurable delimiter,
//! optionally trimming whitespace and skipping empty tokens.

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct SplitConfig {
    /// Delimiter string used to split each line.
    pub delimiter: String,
    /// Trim whitespace from each token.
    pub trim: bool,
    /// Drop empty tokens that result from consecutive delimiters.
    pub skip_empty: bool,
    /// Maximum number of tokens to produce (0 = unlimited).
    pub max_tokens: usize,
}

impl Default for SplitConfig {
    fn default() -> Self {
        Self {
            delimiter: " ".to_string(),
            trim: true,
            skip_empty: true,
            max_tokens: 0,
        }
    }
}

/// Splits `line` according to `config` and returns the resulting tokens.
///
/// When `max_tokens` is non-zero the last token contains the remainder of
/// the line (including any delimiters that were not consumed).
pub fn split_line<'a>(line: &'a str, config: &SplitConfig) -> Vec<Cow<'a, str>> {
    let mut tokens: Vec<Cow<'a, str>> = Vec::new();

    if config.max_tokens == 1 {
        let t = maybe_trim(line, config.trim);
        if !t.is_empty() || !config.skip_empty {
            tokens.push(t);
        }
        return tokens;
    }

    let limit = if config.max_tokens == 0 {
        usize::MAX
    } else {
        config.max_tokens - 1
    };

    let mut remainder = line;
    let mut count = 0usize;

    while !remainder.is_empty() && count < limit {
        match remainder.find(config.delimiter.as_str()) {
            Some(pos) => {
                let part = &remainder[..pos];
                remainder = &remainder[pos + config.delimiter.len()..];
                let t = maybe_trim(part, config.trim);
                if !t.is_empty() || !config.skip_empty {
                    tokens.push(t);
                    count += 1;
                }
            }
            None => {
                break;
            }
        }
    }

    // Push whatever is left as the final token.
    let tail = maybe_trim(remainder, config.trim);
    if !tail.is_empty() || !config.skip_empty {
        tokens.push(tail);
    }

    tokens
}

fn maybe_trim(s: &str, trim: bool) -> Cow<'_, str> {
    if trim {
        let trimmed = s.trim();
        if trimmed.len() == s.len() {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(trimmed.to_string())
        }
    } else {
        Cow::Borrowed(s)
    }
}
