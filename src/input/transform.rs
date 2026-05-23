//! Line transformation: apply find-and-replace or regex substitution to log lines.

use regex::Regex;

#[derive(Debug, Clone)]
pub enum TransformRule {
    Replace { from: String, to: String },
    RegexReplace { pattern: Regex, replacement: String },
    Uppercase,
    Lowercase,
    Trim,
}

#[derive(Debug, Clone)]
pub struct LineTransformer {
    rules: Vec<TransformRule>,
}

impl LineTransformer {
    pub fn new(rules: Vec<TransformRule>) -> Self {
        Self { rules }
    }

    pub fn transform(&self, line: &str) -> String {
        let mut result = line.to_string();
        for rule in &self.rules {
            result = apply_rule(&result, rule);
        }
        result
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

fn apply_rule(line: &str, rule: &TransformRule) -> String {
    match rule {
        TransformRule::Replace { from, to } => line.replace(from.as_str(), to.as_str()),
        TransformRule::RegexReplace { pattern, replacement } => {
            pattern.replace_all(line, replacement.as_str()).into_owned()
        }
        TransformRule::Uppercase => line.to_uppercase(),
        TransformRule::Lowercase => line.to_lowercase(),
        TransformRule::Trim => line.trim().to_string(),
    }
}

impl Default for LineTransformer {
    fn default() -> Self {
        Self::new(vec![])
    }
}

#[cfg(test)]
#[path = "transform_tests.rs"]
mod tests;
