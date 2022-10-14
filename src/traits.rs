pub trait ReadLinesIntoStringOnSelf {
    fn read_lines_into_string(self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringOnRefSelf {
    fn read_lines_into_string(&self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringWithClipOnSelf {
    fn read_lines_into_string_with_clip(self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringWithClipOnRefSelf {
    fn read_lines_into_string_with_clip(&self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringWithTrimOnSelf {
    fn read_lines_into_string_with_trim(self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringWithTrimOnRefSelf {
    fn read_lines_into_string_with_trim(&self) -> ::std::io::Result<String>;
}

pub trait ReadLinesIntoStringsOnSelf {
    fn read_lines_into_vec_string(self) -> ::std::io::Result<Vec<String>>;
}

pub trait ReadLinesIntoStringsOnRefSelf {
    fn read_lines_into_vec_string(&self) -> ::std::io::Result<Vec<String>>;
}

pub trait ReadLinesIntoStringsWithClipOnSelf {
    fn read_lines_into_vec_string_with_clip(self) -> ::std::io::Result<Vec<String>>;
}

pub trait ReadLinesIntoStringsWithClipOnRefSelf {
    fn read_lines_into_vec_string_with_clip(&self) -> ::std::io::Result<Vec<String>>;
}

pub trait ReadLinesIntoStringsWithTrimOnSelf {
    fn read_lines_into_vec_string_with_trim(self) -> ::std::io::Result<Vec<String>>;
}

pub trait ReadLinesIntoStringsWithTrimOnRefSelf {
    fn read_lines_into_vec_string_with_trim(&self) -> ::std::io::Result<Vec<String>>;
}
