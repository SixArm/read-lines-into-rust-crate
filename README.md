# read_lines_into Rust crate

Read lines (from Path, File, BufRead) into a struct (String, Vec<String>).

Example:

```rust
// Choose any existing text file
let path = Path::new("example.txt");
 
// Read lines from the path's file into a string
let string = path.read_lines_into_string().unwrap();
 
// Read lines from the path's file into a vector of strings
let strings = path.read_lines_into_vec_string().unwrap();
```


## Install

Add dependency:

```toml
[dependencies]
read_lines_into = "*"
```

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

* Project: read-lines-into-rust-crate
* Version: 1.0.0
* Created: 2022-10-01T22:58:34Z
* Updated: 2022-10-14T01:07:18Z
* Website: https://github.com/sixarm/read-lines-into-rust-crate
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
* License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0
