//! Automatic schema detection pipeline stage.
//!
//! Buffers a configurable number of lines, runs schema inference,
//! then emits both the schema and the original lines downstream.

use super::schema::Schema;

#[derive(Debug)]
pub struct SchemaDetector {
    sample_size: usize,
    buffer: Vec<String>,
    schema: Option<Schema>,
    emitted_buffer: bool,
}

impl SchemaDetector {
    pub fn new(sample_size: usize) -> Self {
        Self {
            sample_size,
            buffer: Vec::with_capacity(sample_size),
            schema: None,
            emitted_buffer: false,
        }
    }

    /// Feed a line into the detector. Returns lines ready for downstream
    /// processing. After the sample window is full the schema is locked.
    pub fn feed(&mut self, line: String) -> Vec<String> {
        if self.schema.is_none() {
            self.buffer.push(line);
            if self.buffer.len() >= self.sample_size {
                self.finalize_schema();
                let drained: Vec<String> = self.buffer.drain(..).collect();
                self.emitted_buffer = true;
                return drained;
            }
            return vec![];
        }
        vec![line]
    }

    /// Flush remaining buffered lines (called at end of input).
    pub fn flush(&mut self) -> Vec<String> {
        if self.schema.is_none() && !self.buffer.is_empty() {
            self.finalize_schema();
        }
        self.buffer.drain(..).collect()
    }

    pub fn schema(&self) -> Option<&Schema> {
        self.schema.as_ref()
    }

    pub fn is_ready(&self) -> bool {
        self.schema.is_some()
    }

    fn finalize_schema(&mut self) {
        let refs: Vec<&str> = self.buffer.iter().map(String::as_str).collect();
        let mut s = Schema::new();
        s.infer_from_lines(&refs);
        self.schema = Some(s);
    }
}
