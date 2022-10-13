use std::path::Path;
use std::fs::File;
use crate::traits::*;

impl ReadLinesIntoStringOnRefSelf for Path {

    /// Read lines into String; keep each line end `\n` or `\r\n`.
    /// 
    /// ```
    /// use std::path::Path;
    /// use std::fs::File;
    /// use read_lines_into_string::traits::*;
    /// 
    /// let path = Path::new("example.txt");
    /// let string: String = path.read_lines_into_string().unwrap();
    /// ```
    /// 
    /// Any error will return immediately.
    /// 
    fn read_lines_into_string(&self) -> ::std::io::Result<String> {
        let file = File::open(&self)?;
        file.read_lines_into_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_lf() {
        let path = Path::new("example.txt");
        assert_eq!(
            path.read_lines_into_string().unwrap(), 
            "lorem\nipsum\n"
        );
    }

    #[test]
    fn with_crlf() {
        let path = Path::new("example-with-crlf.txt");
        assert_eq!(
            path.read_lines_into_string().unwrap(), 
            "lorem\r\nipsum\r\n"
        );
    }

    #[test]
    fn with_indent() {
        let path = Path::new("example-with-indent.txt");
        assert_eq!(
            path.read_lines_into_string().unwrap(),
            "    lorem\n    ipsum\n"
        );
    }

}
