use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, HighlighterError>;

#[derive(Debug)]
pub enum HighlighterError {
    UnknownCaptureName,
}

impl std::fmt::Display for HighlighterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            HighlighterError::UnknownCaptureName => {
                write!(f, "highlight: unknown capture name encountered")
            }
        }
    }
}
