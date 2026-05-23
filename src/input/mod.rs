pub mod reader;
pub mod glob;
pub mod decompressor;
pub mod line_counter;
pub mod sampler;
pub mod encoding;
pub mod field_extractor;
pub mod field_filter;
pub mod rate_limiter;
pub mod schema;
pub mod schema_detector;
pub mod checkpoint;
pub mod timeout;
pub mod batch;

#[cfg(test)]
mod reader_tests;
#[cfg(test)]
mod glob_tests;
#[cfg(test)]
mod decompressor_tests;
#[cfg(test)]
mod line_counter_tests;
#[cfg(test)]
mod sampler_tests;
#[cfg(test)]
mod encoding_tests;
#[cfg(test)]
mod field_extractor_tests;
#[cfg(test)]
mod field_filter_tests;
#[cfg(test)]
mod rate_limiter_tests;
#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod schema_detector_tests;
#[cfg(test)]
mod checkpoint_tests;
#[cfg(test)]
mod timeout_tests;
#[cfg(test)]
mod batch_tests;
