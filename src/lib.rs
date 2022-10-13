//! # read_lines_into_string Rust crate
//! 
//! Read lines (from a Path or File etc.) into a String or Vec<String>.
//! 
//! Examples:
//! 
//! ```rust
//! use std::path::Path;
//! use read_lines_into_string::traits::*;
//! 
//! // Choose any existing text file
//! let path = Path::new("example.txt");
//! 
//! // Read lines from the path's file into a string
//! let string = path.read_lines_into_string().unwrap();
//! 
//! // Read lines from the path's file into a vector of strings
//! let strings = path.read_lines_into_strings().unwrap();
//! ```
//! 
//! ## Install
//! 
//! You can use this Rust crate:
//! 
//! ```toml
//! [dependencies]
//! read_lines_from_file_into_string = "*"
//! ```
//! 
//! ## Notes
//! 
//! These functions are written to be easy to understand,
//! so you can copy them into your own code if you wish.
//! 
//! These functions use buffered readers for efficiency.
//! 
//! If you're reading very large files, then you may prefer
//! to write your own code to process each line as it's read.
//! 
//! ## Line endings using LF and CRLF
//! 
//! Unix systems typically end text lines with `\n` LINE FEED (LF).
//! 
//! Windows systems typically end text lines with `\r` CARRIAGE RETURN (CR)
//! then `\n` LINE FEED (LF).
//! 
//! 
//! ## Tests
//! 
//! Test files have 3 content variations:
//!
//!   * example.txt has lines that end with LF.
//!   * example-with-crlf.txt has lines that end wit CRLF.
//!   * example-with-indent.txt has lines with leading spaces.
//!
//! ## Tracking
//! 
//! * Project: file-lines-into-string-rust-crate
//! * Version: 2.0.0
//! * Created: 2022-10-01T22:58:34Z
//! * Updated: 2022-10-12T21:56:45Z
//! * Website: https://github.com/sixarm/file-into-string-rust-crate
//! * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
//! * License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0

// use std::fs::File;
// use std::io::BufRead;
// use std::io::BufReader;
// use std::path::Path;

pub mod traits;
pub mod buf_reader {
    pub mod read_lines_into_string;
    pub mod read_lines_into_string_with_clip;
    pub mod read_lines_into_string_with_trim;
    pub mod read_lines_into_strings;
    pub mod read_lines_into_strings_with_clip;
    pub mod read_lines_into_strings_with_trim;
}
pub mod file {
    pub mod read_lines_into_string;
    pub mod read_lines_into_string_with_clip;
    pub mod read_lines_into_string_with_trim;
    pub mod read_lines_into_strings;
    pub mod read_lines_into_strings_with_clip;
    pub mod read_lines_into_strings_with_trim;
}
pub mod path {
    pub mod read_lines_into_string;
    pub mod read_lines_into_string_with_clip;
    pub mod read_lines_into_string_with_trim;
    pub mod read_lines_into_strings;
    pub mod read_lines_into_strings_with_clip;
    pub mod read_lines_into_strings_with_trim;
}
