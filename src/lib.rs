//! # bhswz-rs
//!
//! A Rust library for reading and writing Brawlhalla's swz files.
//!
//! # Organization
//!
//! This library exports one main module, `bhswz`, which contains the following components:
//! * `SwzReader`: A reader that can dump SWZ files. The 'new' function takes a reader and the SWZ key.
//! * `SwzWriter`: A writer that can repack the contents of an SWZ file. The 'new' function takes a writer, the SWZ key, and an optional seed.
//! * `get_swz_file_name`: A function to extract the file name from an SWZ file's content.

mod bhswz;
pub use bhswz::{SwzReader, SwzWriter, get_swz_file_name};