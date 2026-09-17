use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T,ParserError>;

#[derive(Debug)]
pub enum ParserError {
    SetLanguage,
    Parse
}

impl std::fmt::Display for ParserError {
    fn fmt(&self,f: &mut Formatter<'_>) -> std::result::Result<(),std::fmt::Error> {
       match self {
            ParserError::SetLanguage => {
                write!(f,"parser.set_language: couldn't convert the language")
            }
            ParserError::Parse => {
                write!(f,"parser: couldn't parse the source code")
            }
        } 
    }
}
