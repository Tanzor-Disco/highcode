use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, SourceError>;

#[derive(Debug)]
pub enum SourceError {
    ReadExtension,
    ReadFile,
}

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            SourceError::ReadExtension => {
                write!(f, "get_language: couldn't read the extension of the file")
            }
            SourceError::ReadFile => {
                write!(f, "read_file: couldn't read the contents of the file")
            }
        }
    }
}
