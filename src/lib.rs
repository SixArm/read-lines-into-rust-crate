//! # file_into_string Rust crate
//! 
//! Read a typical text file into a string or vector of strings.
//! 
//! * `file_into_string(file: File) -> std::io::Result<String>`
//! 
//! * `file_into_strings(file: File) -> std::io::Result<Vec<String>>`
//! 
//! Examples:
//! 
//! ```rust
//! use std::fs::File;
//! use file_into_string::*;
//! 
//! // Open an existing text file; read the File into a String.
//! let file = File::open("example.txt").unwrap();
//! let string = file_into_string(file).unwrap();
//! 
//! // Open an existing text file; read the File into a Vec<String>.
//! let file = File::open("example.txt").unwrap();
//! let strings = file_into_strings(file).unwrap();
//! ```
//! 
//! ## Install
//! 
//! You can use this Rust crate:
//! 
//! ```toml
//! [dependencies]
//! file_into_string = "*"
//! ```
//! 
//! Or if you prefer, you can copy the source code into your own program.
//! 
//! ## Notes
//! 
//! These functions deliberately preserve line endings,
//! which are `\n` newlines and `\r` carriage returns.
//! 
//! These functions use buffered readers for efficiency.
//! 
//! These functions are written to be easy to understand,
//! so you can copy them into your own code if you wish.
//! 
//! If you're reading very large files, then you may prefer
//! to write your own code to process each line as it's read.
//! 
//! ## Tracking
//! 
//! * Project: file-into-string-rust-crate
//! * Version: 1.1.1
//! * Created: 2022-10-01T22:58:34Z
//! * Updated: 2022-10-12T21:56:45Z
//! * Website: https://github.com/sixarm/file-into-string-rust-crate
//! * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
//! * License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0
  
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Read a File into a String.
/// 
/// ```
/// use std::fs::File;
/// use file_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = file_into_string(file).unwrap();
/// ```
/// 
/// Note: this function deliberately preserves line endings,
/// which are `\n` newlines and `\r` carriage returns.
/// 
pub fn file_into_string(file: File) -> ::std::io::Result<String> {
    let mut string = String::new();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut n: usize;
    loop {
        n = reader.read_line(&mut buf)?;
        if n == 0 { break; }
        string.push_str(&buf);
        buf.clear();
    }
    Ok(string)
}

/// Read a File into a Vec<String>.
/// 
/// ```
/// use std::fs::File;
/// use file_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let strings: Vec<String> = file_into_strings(file).unwrap();
/// ```
/// 
/// Note: this function deliberately preserves line endings,
/// which are `\n` newlines and `\r` carriage returns.
/// 
pub fn file_into_strings(file: File) -> ::std::io::Result<Vec<String>> {
    let mut strings = Vec::<String>::new();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut n: usize;
    loop {
        n = reader.read_line(&mut buf)?;
        if n == 0 { break; }
        strings.push(buf.clone());
        buf.clear();
    }
    Ok(strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    static PATH: &str = "example.txt"; // The contents must be "lorem\nipsum\n"

    #[test]
    fn test_file_into_string() {
        let file = std::fs::File::open(PATH).unwrap();
        let s = file_into_string(file).unwrap();
        assert_eq!(s, "lorem\nipsum\n");
    }

    #[test]
    fn test_file_into_strings() {
        let file = std::fs::File::open(PATH).unwrap();
        let actual_strings = file_into_strings(file).unwrap();
        let expect_strings = vec![String::from("lorem\n"), String::from("ipsum\n")];
        assert_eq!(actual_strings, expect_strings);
    }

}
