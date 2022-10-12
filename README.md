# file_into_string Rust crate

Read a typical text file into a string or vector of strings.

* `file_into_string(file: File) -> std::io::Result<String>`

* `file_into_strings(file: File) -> std::io::Result<Vec<String>>`

You can use this Rust crate:

```toml
[dependencies]
file_into_string = "*"
```

Or if you prefer, you can copy the source code into your own program.
The code is short, easy to understand, and easy to change as you like.


Examples:

```rust
use std::fs::File;
use file_into_string::*;

// Open an existing text file; read the file into one string.
let file: File = File::open("example.txt").unwrap();
let string: String = file_into_string(file).unwrap();
 
// Open an existing text file; read the file into a vector of strings.
let file: File = File::open("example.txt").unwrap();
let strings: Vec<String> = file_into_strings(file).unwrap();
```

## Notes

These functions deliberately preserve line endings,
which are `\n` newlines and `\r` carriage returns.

These functions use buffered readers for efficiency.

These functions are written to be easy to understand,
so you can copy them into your own code if you wish.

If you're reading very large files, then you may prefer
to write your own code to process each line as it's read.

## Tracking

* Project: file-into-string-rust-crate
* Version: 1.0.0
* Created: 2022-10-01T22:58:34Z
* Updated: 2022-10-12T21:15:45Z
* Website: https://github.com/sixarm/file-into-string-rust-crate
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
* License: MIT OR Apache-2.0 OR GPL-2.0 OR GPL-3.0
