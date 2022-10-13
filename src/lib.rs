//! # file_lines_into_string Rust crate
//! 
//! Read a typical text file into a string or vector of strings.
//! 
//! Examples:
//! 
//! ```rust
//! use std::fs::File;
//! use file_lines_into_string::*;
//! 
//! // Open an existing text file; read the File into a String.
//! let file = File::open("example.txt").unwrap();
//! let string = file_lines_into_string(file).unwrap();
//! 
//! // Open an existing text file; read the File into a Vec<String>.
//! let file = File::open("example.txt").unwrap();
//! let strings = file_lines_into_strings(file).unwrap();
//! ```
//! 
//! ## Functions
//! 
//! * `file_lines_into_string(file: File) -> std::io::Result<String>`
//! 
//! * `file_lines_into_string_clip(file: File) -> std::io::Result<String>`
//! 
//! * `file_lines_into_string_trim(file: File) -> std::io::Result<String>`
//! 
//! * `file_lines_into_strings(file: File) -> std::io::Result<Vec<String>>`
//! 
//! * `file_lines_into_strings_clip(file: File) -> std::io::Result<Vec<String>>`
//! 
//! * `file_lines_into_strings_trip(file: File) -> std::io::Result<Vec<String>>`
//! 
//! ## Install
//! 
//! You can use this Rust crate:
//! 
//! ```toml
//! [dependencies]
//! file_lines_into_string = "*"
//! ```
//! 
//! Or if you prefer, you can copy the source code into your own program.
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
//! ## Tracking
//! 
//! * Project: file-lines-into-string-rust-crate
//! * Version: 2.0.0
//! * Created: 2022-10-01T22:58:34Z
//! * Updated: 2022-10-12T21:56:45Z
//! * Website: https://github.com/sixarm/file-into-string-rust-crate
//! * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
//! * License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0
  
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Read File lines into a String; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = file_lines_into_string(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_string(file: File) -> ::std::io::Result<String> {
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

/// Read File lines into a String; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = file_lines_into_string_clip(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_string_clip(file: File) -> ::std::io::Result<String> { 
    let mut string = String::new();
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let x = line?;
        string.push_str(&x);
    }
    Ok(string)
}

/// Read File lines into a String; trim each line of whitespace.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = file_lines_into_string_trim(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_string_trim(file: File) -> ::std::io::Result<String> { 
    let mut string = String::new();
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let x = line?;
        string.push_str(&x.trim());
    }
    Ok(string)
}

/// Read File lines into a Vec<String>; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let strings: Vec<String> = file_lines_into_strings(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_strings(file: File) -> ::std::io::Result<Vec<String>> {
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

/// Read File lines into a Vec<String>; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let strings: Vec<String> = file_lines_into_strings(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_strings_clip(file: File) -> ::std::io::Result<Vec<String>> { 
    let mut strings = Vec::<String>::new();
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let x = line?;
        strings.push(x);
    }
    Ok(strings)
}

/// Read File lines into a Vec<String>; trim each line of whitepace.
/// 
/// ```
/// use std::fs::File;
/// use file_lines_into_string::*;
/// 
/// let file: File = File::open("example-with-indent.txt").unwrap();
/// let strings: Vec<String> = file_lines_into_strings_trim(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn file_lines_into_strings_trim(file: File) -> ::std::io::Result<Vec<String>> { 
    let mut strings = Vec::<String>::new();
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        let x = line?;
        strings.push(String::from(x.trim()));
    }
    Ok(strings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_lines_into_string() {
        let file = std::fs::File::open("example.txt").unwrap();
        let s = file_lines_into_string(file).unwrap();
        assert_eq!(s, "lorem\nipsum\n");
    }

    #[test]
    fn test_file_lines_into_string_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let s = file_lines_into_string(file).unwrap();
        assert_eq!(s, "lorem\r\nipsum\r\n");
    }

    #[test]
    fn test_file_lines_into_string_x_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let s = file_lines_into_string(file).unwrap();
        assert_eq!(s, "    lorem\n    ipsum\n");
    }

    #[test]
    fn test_file_lines_into_string_clip() {
        let file = std::fs::File::open("example.txt").unwrap();
        let s = file_lines_into_string_clip(file).unwrap();
        assert_eq!(s, "loremipsum");
    }

    #[test]
    fn test_file_lines_into_string_clip_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let s = file_lines_into_string_clip(file).unwrap();
        assert_eq!(s, "loremipsum");
    }

    #[test]
    fn test_file_lines_into_string_clip_x_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let s = file_lines_into_string_clip(file).unwrap();
        assert_eq!(s, "    lorem    ipsum");
    }

    #[test]
    fn test_file_lines_into_string_trim() {
        let file = std::fs::File::open("example.txt").unwrap();
        let s = file_lines_into_string_clip(file).unwrap();
        assert_eq!(s, "loremipsum");
    }

    #[test]
    fn test_file_lines_into_string_trim_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let s = file_lines_into_string_clip(file).unwrap();
        assert_eq!(s, "loremipsum");
    }

    #[test]
    fn test_file_lines_into_string_trim_x_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let s = file_lines_into_string_trim(file).unwrap();
        assert_eq!(s, "loremipsum");
    }

    #[test]
    fn test_file_lines_into_strings() {
        let file = std::fs::File::open("example.txt").unwrap();
        let actual_strings = file_lines_into_strings(file).unwrap();
        let expect_strings = vec![String::from("lorem\n"), String::from("ipsum\n")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let actual_strings = file_lines_into_strings(file).unwrap();
        let expect_strings = vec![String::from("lorem\r\n"), String::from("ipsum\r\n")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_x_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let actual_strings = file_lines_into_strings(file).unwrap();
        let expect_strings = vec![String::from("    lorem\n"), String::from("    ipsum\n")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_clip() {
        let file = std::fs::File::open("example.txt").unwrap();
        let actual_strings = file_lines_into_strings_clip(file).unwrap();
        let expect_strings = vec![String::from("lorem"), String::from("ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_clip_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let actual_strings = file_lines_into_strings_clip(file).unwrap();
        let expect_strings = vec![String::from("lorem"), String::from("ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_clip_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let actual_strings = file_lines_into_strings_clip(file).unwrap();
        let expect_strings = vec![String::from("    lorem"), String::from("    ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_trim() {
        let file = std::fs::File::open("example.txt").unwrap();
        let actual_strings = file_lines_into_strings_trim(file).unwrap();
        let expect_strings = vec![String::from("lorem"), String::from("ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_trim_x_crlf() {
        let file = std::fs::File::open("example-with-crlf.txt").unwrap();
        let actual_strings = file_lines_into_strings_trim(file).unwrap();
        let expect_strings = vec![String::from("lorem"), String::from("ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

    #[test]
    fn test_file_lines_into_strings_trim_x_indent() {
        let file = std::fs::File::open("example-with-indent.txt").unwrap();
        let actual_strings = file_lines_into_strings_trim(file).unwrap();
        let expect_strings = vec![String::from("lorem"), String::from("ipsum")];
        assert_eq!(actual_strings, expect_strings);
    }

}
