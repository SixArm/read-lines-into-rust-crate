use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::traits::*;

impl ReadLinesIntoStringOnSelf for BufReader<File> {

    /// Read lines into String; keep each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use read_lines_into_string::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let mut buf_reader = BufReader::new(file);
    /// let string: String = buf_reader.read_lines_into_string().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_string(mut self) -> ::std::io::Result<String> {
        let mut string = String::new();
        let mut buf = String::new();
        let mut n: usize;
        loop {
            n = self.read_line(&mut buf)?;
            if n == 0 { break; }
            string.push_str(&buf);
            buf.clear();
        }
        Ok(string)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn sut(path_str: &str) -> BufReader<File> {
        BufReader::new(File::open(path_str).unwrap())
    }

    #[test]
    fn with_lf() {
        let buf_reader = sut("example.txt");
        assert_eq!(
            buf_reader.read_lines_into_string().unwrap(),
            "lorem\nipsum\n"
        );
    }

    #[test]
    fn with_crlf() {
        let buf_reader = sut("example-with-crlf.txt");
        assert_eq!(
            buf_reader.read_lines_into_string().unwrap(),
            "lorem\r\nipsum\r\n"
        );
    }

    #[test]
    fn with_indent() {
        let buf_reader = sut("example-with-indent.txt");
        assert_eq!(
            buf_reader.read_lines_into_string().unwrap(),
            "    lorem\n    ipsum\n"
        );
    }

}
