use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::traits::*;

impl ReadLinesIntoStringsWithTrimOnSelf for BufReader<File> {

    /// Read lines into Vec<String>; trim each line of whitespace.
    /// 
    /// ```
    /// use std::fs::File;
    /// use std::io::BufReader;
    /// use read_lines_into::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let mut buf_reader = BufReader::new(file);
    /// let strings: Vec<String> = buf_reader.read_lines_into_vec_string_with_trim().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_vec_string_with_trim(self) -> ::std::io::Result<Vec<String>> {
        let mut strings = Vec::<String>::new();
        let lines = self.lines();
        for line in lines {
            let x = line?;
            strings.push(String::from(x.trim()));
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
            buf_reader.read_lines_into_vec_string_with_trim().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_crlf() {
        let buf_reader = sut("example-with-crlf.txt");
        assert_eq!(
            buf_reader.read_lines_into_vec_string_with_trim().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_indent() {
        let buf_reader = sut("example-with-indent.txt");
        assert_eq!(
            buf_reader.read_lines_into_vec_string_with_trim().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

}
