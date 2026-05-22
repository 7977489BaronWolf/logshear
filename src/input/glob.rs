use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub struct GlobExpander {
    base_dir: PathBuf,
}

impl GlobExpander {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    /// Expand a glob pattern or plain path into a sorted list of matching file paths.
    pub fn expand(&self, pattern: &str) -> Result<Vec<PathBuf>, GlobError> {
        if is_glob(pattern) {
            self.expand_glob(pattern)
        } else {
            let path = self.base_dir.join(pattern);
            if path.exists() {
                Ok(vec![path])
            } else {
                Err(GlobError::NotFound(pattern.to_string()))
            }
        }
    }

    fn expand_glob(&self, pattern: &str) -> Result<Vec<PathBuf>, GlobError> {
        let full_pattern = self.base_dir.join(pattern);
        let pattern_str = full_pattern
            .to_str()
            .ok_or_else(|| GlobError::InvalidPattern(pattern.to_string()))?;

        let mut paths: Vec<PathBuf> = fs::read_dir(&self.base_dir)
            .map_err(|e| GlobError::Io(e.to_string()))?
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .filter(|p| matches_glob(p, pattern_str))
            .collect();

        paths.sort();
        Ok(paths)
    }
}

fn is_glob(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

fn matches_glob(path: &Path, pattern: &str) -> bool {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => return false,
    };
    glob_match(pattern, path_str)
}

/// Minimal glob matcher supporting `*` and `?` wildcards.
fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    glob_match_inner(&p, &t)
}

fn glob_match_inner(p: &[char], t: &[char]) -> bool {
    match (p.first(), t.first()) {
        (None, None) => true,
        (Some(&'*'), _) => {
            // match zero or more characters
            glob_match_inner(&p[1..], t)
                || (!t.is_empty() && glob_match_inner(p, &t[1..]))
        }
        (Some(&'?'), Some(_)) => glob_match_inner(&p[1..], &t[1..]),
        (Some(pc), Some(tc)) if pc == tc => glob_match_inner(&p[1..], &t[1..]),
        _ => false,
    }
}

#[derive(Debug)]
pub enum GlobError {
    NotFound(String),
    InvalidPattern(String),
    Io(String),
}

impl std::fmt::Display for GlobError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobError::NotFound(p) => write!(f, "path not found: {}", p),
            GlobError::InvalidPattern(p) => write!(f, "invalid glob pattern: {}", p),
            GlobError::Io(e) => write!(f, "io error: {}", e),
        }
    }
}
