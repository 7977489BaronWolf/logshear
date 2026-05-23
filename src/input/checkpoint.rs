//! Checkpoint support for resumable log processing.
//!
//! Tracks the last successfully processed byte offset per file,
//! allowing logshear to resume from where it left off.

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

/// Stores per-file byte offsets for resumable processing.
#[derive(Debug, Default)]
pub struct Checkpoint {
    offsets: HashMap<PathBuf, u64>,
    checkpoint_path: Option<PathBuf>,
}

impl Checkpoint {
    /// Create an in-memory checkpoint (not persisted).
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a checkpoint from a file, or create a fresh one if it doesn't exist.
    pub fn load(path: &Path) -> io::Result<Self> {
        let mut offsets = HashMap::new();
        if path.exists() {
            let file = fs::File::open(path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((raw_path, raw_offset)) = line.split_once('\t') {
                    if let Ok(offset) = raw_offset.trim().parse::<u64>() {
                        offsets.insert(PathBuf::from(raw_path.trim()), offset);
                    }
                }
            }
        }
        Ok(Self {
            offsets,
            checkpoint_path: Some(path.to_path_buf()),
        })
    }

    /// Return the saved offset for a file, or 0 if none.
    pub fn offset_for(&self, path: &Path) -> u64 {
        self.offsets.get(path).copied().unwrap_or(0)
    }

    /// Update the offset for a file.
    pub fn set_offset(&mut self, path: &Path, offset: u64) {
        self.offsets.insert(path.to_path_buf(), offset);
    }

    /// Persist the checkpoint to disk (no-op for in-memory checkpoints).
    pub fn save(&self) -> io::Result<()> {
        let Some(ref cp_path) = self.checkpoint_path else {
            return Ok(());
        };
        let mut file = fs::File::create(cp_path)?;
        writeln!(file, "# logshear checkpoint")?;
        for (path, offset) in &self.offsets {
            writeln!(file, "{}\t{}", path.display(), offset)?;
        }
        Ok(())
    }

    /// Number of tracked files.
    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}
