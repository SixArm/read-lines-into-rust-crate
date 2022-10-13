//! # read_lines_into_string Rust crate
//! 
//! Read lines of a Path, File, BufRead, into a String or Vec<String>.
//! 
//! Examples:
//! 
//! ```rust
//! use std::fs::File;
//! use read_lines_into_string::*;
//! 
//! // Open an existing text file; read the File into a String.
//! let file = File::open("example.txt").unwrap();
//! let string = read_lines_from_file_into_string(file).unwrap();
//! 
//! // Open an existing text file; read the File into a Vec<String>.
//! let file = File::open("example.txt").unwrap();
//! let strings = read_lines_from_file_into_strings(file).unwrap();
//! ```
//! 
//! ## Functions
//! 
//! * `read_lines_from_file_into_string(file: File) -> std::io::Result<String>`
//! 
//! * `read_lines_from_file_into_string_clip(file: File) -> std::io::Result<String>`
//! 
//! * `read_lines_from_file_into_string_trim(file: File) -> std::io::Result<String>`
//! 
//! * `read_lines_from_file_into_strings(file: File) -> std::io::Result<Vec<String>>`
//! 
//! * `read_lines_from_file_into_strings_clip(file: File) -> std::io::Result<Vec<String>>`
//! 
//! * `read_lines_from_file_into_strings_trip(file: File) -> std::io::Result<Vec<String>>`
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
use std::path::Path;

/// Read lines from Path into String; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let string: String = read_lines_from_path_into_string(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_string(path: &Path) -> ::std::io::Result<String> {
    let file = File::open(path)?;
    read_lines_from_file_into_string(file)
}
/// Read lines from File into String; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = read_lines_from_file_into_string(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_string(file: File) -> ::std::io::Result<String> {
    let reader = BufReader::new(file);
    read_lines_into_string(reader)
}

/// Read lines from BufRead into String; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let string: String = read_lines_into_string(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_string(mut reader: impl BufRead) -> ::std::io::Result<String> {
    let mut string = String::new();
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

/// Read lines from Path into String; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let string: String = read_lines_from_path_into_string_clip(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_string_clip(path: &Path) -> ::std::io::Result<String> { 
    let file = File::open(path)?;
    read_lines_from_file_into_string_clip(file)
}

/// Read lines from File into String; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = read_lines_from_file_into_string_clip(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_string_clip(file: File) -> ::std::io::Result<String> { 
    let reader = std::io::BufReader::new(file);
    read_lines_into_string_clip(reader)
}

/// Read lines from BufRead into String; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let string: String = read_lines_into_string_clip(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_string_clip(reader: impl BufRead) -> ::std::io::Result<String> { 
    let mut string = String::new();
    let lines = reader.lines();
    for line in lines {
        let x = line?;
        string.push_str(&x);
    }
    Ok(string)
}

/// Read lines from Path into String; trim each line of whitespace.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let string: String = read_lines_from_path_into_string_trim(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_string_trim(path: &Path) -> ::std::io::Result<String> { 
    let file = File::open(path)?;
    read_lines_from_file_into_string_trim(file)
}

/// Read lines from File into String; trim each line of whitespace.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let string: String = read_lines_from_file_into_string_trim(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_string_trim(file: File) -> ::std::io::Result<String> { 
    let reader = std::io::BufReader::new(file);
    read_lines_into_string_trim(reader)
}

/// Read lines from BufRead into String; trim each line of whitespace.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let string: String = read_lines_into_string_trim(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_string_trim(reader: impl BufRead) -> ::std::io::Result<String> { 
    let mut string = String::new();
    let lines = reader.lines();
    for line in lines {
        let x = line?;
        string.push_str(&x.trim());
    }
    Ok(string)
}

/// Read lines from Path into Vec<String>; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let strings: Vec<String> = read_lines_from_path_into_strings(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_strings(path: &Path) -> ::std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    read_lines_from_file_into_strings(file)
}

/// Read lines from File into Vec<String>; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let strings: Vec<String> = read_lines_from_file_into_strings(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_strings(file: File) -> ::std::io::Result<Vec<String>> {
    let reader = BufReader::new(file);
    read_lines_into_strings(reader)
}

/// Read lines from BufRead into Vec<String>; keep each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let strings: Vec<String> = read_lines_into_strings(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_strings(mut reader: impl BufRead) -> ::std::io::Result<Vec<String>> {
    let mut strings = Vec::<String>::new();
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

/// Read lines from Path into Vec<String>; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let strings: Vec<String> = read_lines_from_path_into_strings(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_strings_clip(path: &Path) -> ::std::io::Result<Vec<String>> { 
    let file = File::open(path)?;
    read_lines_from_file_into_strings_clip(file)
}

/// Read lines from File into Vec<String>; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let strings: Vec<String> = read_lines_from_file_into_strings(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_strings_clip(file: File) -> ::std::io::Result<Vec<String>> { 
    let reader = std::io::BufReader::new(file);
    read_lines_into_strings_clip(reader)
}

/// Read lines from BufRead into Vec<String>; clip each line end `\n` or `\r\n`.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let strings: Vec<String> = read_lines_into_strings(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_strings_clip(reader: impl BufRead) -> ::std::io::Result<Vec<String>> { 
    let mut strings = Vec::<String>::new();
    let lines = reader.lines();
    for line in lines {
        let x = line?;
        strings.push(x);
    }
    Ok(strings)
}

/// Read lines from Path into Vec<String>; trim each line of whitepace.
/// 
/// ```
/// use std::path::Path;
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let path = Path::new("example.txt");
/// let strings: Vec<String> = read_lines_from_path_into_strings_trim(path).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_path_into_strings_trim(path: &Path) -> ::std::io::Result<Vec<String>> { 
    let file = File::open(path)?;
    read_lines_from_file_into_strings_trim(file)
}

/// Read lines from File into Vec<String>; trim each line of whitepace.
/// 
/// ```
/// use std::fs::File;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example-with-indent.txt").unwrap();
/// let strings: Vec<String> = read_lines_from_file_into_strings_trim(file).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_from_file_into_strings_trim(file: File) -> ::std::io::Result<Vec<String>> { 
    let reader = std::io::BufReader::new(file);
    read_lines_into_strings_trim(reader)
}

/// Read lines from BufRead into Vec<String>; trim each line of whitepace.
/// 
/// ```
/// use std::fs::File;
/// use std::io::BufReader;
/// use read_lines_into_string::*;
/// 
/// let file: File = File::open("example-with-indent.txt").unwrap();
/// let mut reader = BufReader::new(file);
/// let strings: Vec<String> = read_lines_into_strings_trim(reader).unwrap();
/// ```
/// 
/// Any error will return immediately.
/// 
pub fn read_lines_into_strings_trim(reader: impl BufRead) -> ::std::io::Result<Vec<String>> { 
    let mut strings = Vec::<String>::new();
    let lines = reader.lines();
    for line in lines {
        let x = line?;
        strings.push(String::from(x.trim()));
    }
    Ok(strings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::path::Path;

    // Test all 54 combinations.
    //
    // Functions have 18 flavors that read:
    //
    //   * from: BufRead | File | Path 
    //   * into: String | Vec<String>
    //   * with: preserve line endings | clip | trim
    //
    // Test text files have 3 flavors:
    //
    //   * example.txt is the main goal and uses LINE FEED
    //   * example-with-crlf.txt has lines with CRLF endings.
    //   * example-with-indent.txt has lines with leading spaces.

    mod read_lines_into_string {
        use super::*;

        mod with_lf {
            use super::*;

            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "lorem\nipsum\n" }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string(path()).unwrap(), expect());
            }
        }

        mod with_crlf {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "lorem\r\nipsum\r\n" }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string(path()).unwrap(), expect());
            }
        }

        mod with_indent {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "    lorem\n    ipsum\n" }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string(path()).unwrap(), expect());
            }
        }
    }

    mod read_lines_into_string_clip {
        use super::*;

        mod with_lf {
            use super::*;

            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "loremipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                    assert_eq!(read_lines_from_path_into_string_clip(path()).unwrap(), expect());
            }
        }

        mod with_crlf {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "loremipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string_clip(path()).unwrap(), expect());
            }
        }

        mod with_indent {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "    lorem    ipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string_clip(path()).unwrap(), expect());
            }
        }
    }

    mod read_lines_into_string_trim {
        use super::*;
    
        mod with_lf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "loremipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
               assert_eq!(read_lines_from_path_into_string_trim(path()).unwrap(), expect());
            }
        }
        
        mod with_crlf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "loremipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string_trim(path()).unwrap(), expect());
            }
        }
        
        mod with_indent {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> &'static str { "loremipsum" }
            
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_string_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_string_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_string_trim(path()).unwrap(), expect());
            }
        }
    }
    
    mod read_lines_into_strings {
        use super::*;

        mod with_lf {
            use super::*;

            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem\n"), String::from("ipsum\n")] }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings(path()).unwrap(), expect());
            }
        }

        mod with_crlf {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem\r\n"), String::from("ipsum\r\n")] }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings(path()).unwrap(), expect());
            }
        }

        mod with_indent {
            use super::*;

            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("    lorem\n"), String::from("    ipsum\n")] }

            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings(path()).unwrap(), expect());
            }
        }
    }

    mod read_lines_into_strings_clip {
        use super::*;
        
        mod with_lf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem"), String::from("ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_clip(path()).unwrap(), expect());
            }
        }
        
        mod with_crlf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem"), String::from("ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_clip(path()).unwrap(), expect());
            }
        }
        
        mod with_indent {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("    lorem"), String::from("    ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_clip(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_clip(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_clip(path()).unwrap(), expect());
            }
        }
    }

    mod read_lines_into_strings_trim {
        use super::*;

        mod with_lf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem"), String::from("ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_trim(path()).unwrap(), expect());
            }
        }
        
        mod with_crlf {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-crlf.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem"), String::from("ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_trim(path()).unwrap(), expect());
            }
        }
        
        mod with_indent {
            use super::*;
        
            fn path() -> &'static Path { Path::new("example-with-indent.txt") }
            fn file() -> File { std::fs::File::open(path()).unwrap() }
            fn reader() -> impl BufRead { BufReader::new(file()) }
            fn expect() -> Vec<String> { vec![String::from("lorem"), String::from("ipsum")] }
        
            #[test]
            fn from_reader() {
                assert_eq!(read_lines_into_strings_trim(reader()).unwrap(), expect());
            }

            #[test]
            fn from_file() {
                assert_eq!(read_lines_from_file_into_strings_trim(file()).unwrap(), expect());
            }

            #[test]
            fn from_path() {
                assert_eq!(read_lines_from_path_into_strings_trim(path()).unwrap(), expect());
            }
        }
    }

}
