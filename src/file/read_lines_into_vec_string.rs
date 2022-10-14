use std::fs::File;
use std::io::BufReader;
use crate::traits::*;

impl ReadLinesIntoStringsOnSelf for File {

    /// Read lines into Vec<String>; keep each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::fs::File;
    /// use read_lines_into::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let strings: Vec<String> = file.read_lines_into_vec_string().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_vec_string(self) -> ::std::io::Result<Vec<String>> {
        let buf_reader = BufReader::new(self);
        buf_reader.read_lines_into_vec_string()
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
            file.read_lines_into_vec_string().unwrap(),
            vec![String::from("lorem\n"), String::from("ipsum\n")]
        );
    }

    #[test]
    fn with_crlf() {
        let file = sut("example-with-crlf.txt");
        assert_eq!(
            file.read_lines_into_vec_string().unwrap(),
            vec![String::from("lorem\r\n"), String::from("ipsum\r\n")]
        );
    }

    #[test]
    fn with_indent() {
        let file = sut("example-with-indent.txt");
        assert_eq!(
            file.read_lines_into_vec_string().unwrap(),
            vec![String::from("    lorem\n"), String::from("    ipsum\n")]
        );
    }

}
