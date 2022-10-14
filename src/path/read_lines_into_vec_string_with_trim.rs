use std::path::Path;
use std::fs::File;
use crate::traits::*;

impl ReadLinesIntoStringsWithTrimOnRefSelf for Path {

    /// Read lines into Vec<String>; trim each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::path::Path;
    /// use std::fs::File;
    /// use read_lines_into::traits::*;
    /// 
    /// let path = Path::new("example.txt");
    /// let strings: Vec<String> = path.read_lines_into_vec_string_with_trim().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_vec_string_with_trim(&self) -> ::std::io::Result<Vec<String>> {
        let file = File::open(&self)?;
        file.read_lines_into_vec_string_with_trim()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_lf() {
        let path = Path::new("example.txt");
        assert_eq!(
            path.read_lines_into_vec_string_with_trim().unwrap(), 
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_crlf() {
        let path = Path::new("example-with-crlf.txt");
        assert_eq!(
            path.read_lines_into_vec_string_with_trim().unwrap(), 
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

    #[test]
    fn with_indent() {
        let path = Path::new("example-with-indent.txt");
        assert_eq!(
            path.read_lines_into_vec_string_with_trim().unwrap(),
            vec![String::from("lorem"), String::from("ipsum")]
        );
    }

}
