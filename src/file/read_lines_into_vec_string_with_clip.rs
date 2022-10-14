use std::fs::File;
use std::io::BufReader;
use crate::traits::*;

impl ReadLinesIntoStringsWithClipOnSelf for File {

    /// Read lines into Vec<String>; clip each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::fs::File;
    /// use read_lines_into_string::traits::*;
    /// 
    /// let file: File = File::open("example.txt").unwrap();
    /// let strings: Vec<String> = file.read_lines_into_vec_string_with_clip().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_vec_string_with_clip(self) -> ::std::io::Result<Vec<String>> {
        let buf_reader = BufReader::new(self);
        buf_reader.read_lines_into_vec_string_with_clip()
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
            file.read_lines_into_vec_string_with_clip().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_crlf() {
        let file = sut("example-with-crlf.txt");
        assert_eq!(
            file.read_lines_into_vec_string_with_clip().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_indent() {
        let file = sut("example-with-indent.txt");
        assert_eq!(
            file.read_lines_into_vec_string_with_clip().unwrap(),
            vec![String::from("    lorem"), String::from("    ipsum")]
        );
    }

}
