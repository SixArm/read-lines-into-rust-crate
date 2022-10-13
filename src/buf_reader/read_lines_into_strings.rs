use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::traits::*;

impl ReadLinesIntoStringsOnSelf for BufReader<File> {

    /// Read lines into Vec<String>; keep each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use read_lines_into_string::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let mut buf_reader = BufReader::new(file);
    /// let strings: Vec<String> = buf_reader.read_lines_into_strings().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_strings(mut self) -> ::std::io::Result<Vec<String>> {
        let mut strings = Vec::<String>::new();
        let mut buf = String::new();
        let mut n: usize;
        loop {
            n = self.read_line(&mut buf)?;
            if n == 0 { break; }
            strings.push(buf.clone());
            buf.clear();
        }
        Ok(strings)
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
            buf_reader.read_lines_into_strings().unwrap(),
            vec![String::from("lorem\n"), String::from("ipsum\n")]
        );
    }

    #[test]
    fn with_crlf() {
        let buf_reader = sut("example-with-crlf.txt");
        assert_eq!(
            buf_reader.read_lines_into_strings().unwrap(),
            vec![String::from("lorem\r\n"), String::from("ipsum\r\n")]
        );
    }

    #[test]
    fn with_indent() {
        let buf_reader = sut("example-with-indent.txt");
        assert_eq!(
            buf_reader.read_lines_into_strings().unwrap(),
            vec![String::from("    lorem\n"), String::from("    ipsum\n")]
        );
    }

}
