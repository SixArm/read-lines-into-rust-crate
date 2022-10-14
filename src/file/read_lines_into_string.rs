use std::fs::File;
use std::io::BufReader;
use crate::traits::*;

impl ReadLinesIntoStringOnSelf for File {

    /// Read lines into String; keep each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::fs::File;
    /// use read_lines_into::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let string: String = file.read_lines_into_string().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_string(self) -> ::std::io::Result<String> {
        let buf_reader = BufReader::new(self);
        buf_reader.read_lines_into_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn sut(path_str: &str) -> File {
        File::open(path_str).unwrap()
    }

    #[test]
    fn with_lf() {
        let file = sut("example.txt");
        assert_eq!(
            file.read_lines_into_string().unwrap(),
            "lorem\nipsum\n"
        );
    }

    #[test]
    fn with_crlf() {
        let file = sut("example-with-crlf.txt");
        assert_eq!(
            file.read_lines_into_string().unwrap(),
            "lorem\r\nipsum\r\n"
        );
    }

    #[test]
    fn with_indent() {
        let file = sut("example-with-indent.txt");
        assert_eq!(
            file.read_lines_into_string().unwrap(),
            "    lorem\n    ipsum\n"
        );
    }

}
