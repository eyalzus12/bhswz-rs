//! # bhswz-rs
//!
//! A Rust library for reading and writing Brawlhalla's swz files.
//!
//! # Organization
//!
//! This library exports the following:
//! * `SwzReader`: A reader that can dump SWZ files. The 'new' function takes a reader and the SWZ key.
//! * `SwzWriter`: A writer that can repack the contents of an SWZ file. The 'new' function takes a writer, the SWZ key, and an optional seed.
//! * `get_swz_file_name`: A function to extract the file name from an SWZ file's content.

mod swz_filename;
mod swz_random;
mod swz_reader;
mod swz_utils;
mod swz_writer;

use swz_random::SwzRandom;
use swz_utils::*;

// Re-exports
pub use swz_filename::get_swz_file_name;
pub use swz_reader::SwzReader;
pub use swz_writer::SwzWriter;
