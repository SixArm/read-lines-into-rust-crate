# read_lines_from_file_into_string Rust crate

Read a typical text file into a string or vector of strings.

Example:

```rust
use std::fs::File;
use read_lines_from_file_into_string::*;

// Open an existing text file.
let file = File::open("example.txt").unwrap();

// Read the File lines into a String.
let string = read_lines_from_file_into_string(file).unwrap();
```

## Functions

* `read_lines_from_file_into_string(file: File) -> std::io::Result<String>`

* `read_lines_from_file_into_string_clip(file: File) -> std::io::Result<String>`

* `read_lines_from_file_into_string_trim(file: File) -> std::io::Result<String>`

* `read_lines_from_file_into_strings(file: File) -> std::io::Result<Vec<String>>`

* `read_lines_from_file_into_strings_clip(file: File) -> std::io::Result<Vec<String>>`

* `read_lines_from_file_into_strings_trip(file: File) -> std::io::Result<Vec<String>>`


## Install

You can use this Rust crate:

```toml
[dependencies]
read_lines_from_file_into_string = "*"
```

Or if you prefer, you can copy the source code into your own program.

## Notes

These functions deliberately preserve line endings,
which are `\n` newlines and `\r` carriage returns.

These functions use buffered readers for efficiency.

These functions are written to be easy to understand,
so you can copy them into your own code if you wish.

If you're reading very large files, then you may prefer
to write your own code to process each line as it's read.

## Line endings using LF and CRLF
 
Unix systems typically end text lines with `\n` LINE FEED (LF).

Windows systems typically end text lines with `\r` CARRIAGE RETURN (CR)
then and `\n` LINE FEED (LF).


## FAQ

### Why use this instead of the Rust `BufRead lines()` function?

Because we have use cases where we must preserve line endings.

### Why publish this as a crate?

Because we want to make it easy to use, and easy to show as examples
for developers who are learning how to program using Rust.

### What are alternatives to consider?

Rust `std::io::BufRead` and its function `lines()`.

Rust `std::include_str` and its macro `include_string!`.

Rust crate `load_file` and its macro `load_str!`.

Rust `std::fs::read_to_string(file_name).unwrap().lines()`.

## Tracking

* Project: file-into-string-rust-crate
* Version: 1.1.1
* Created: 2022-10-01T22:58:34Z
* Updated: 2022-10-12T21:56:45Z
* Website: https://github.com/sixarm/file-into-string-rust-crate
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
* License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0
