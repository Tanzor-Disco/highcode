use crate::error::source::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Language {
    Python,
    Go,
    Rust,
    Csharp,
}

#[derive(Debug)]
pub struct File {
    pub text: String,
    pub language: Language,
}

fn get_language(file_path: &PathBuf) -> Result<Language> {
    let ext = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or(Error::ReadExtension)?;
    match ext {
        "py" => Ok(Language::Python),
        "go" => Ok(Language::Go),
        "rs" => Ok(Language::Rust),
        "cs" => Ok(Language::Csharp),
        _ => Err(Error::ReadExtension),
    }
}

pub fn read_file(file_path: &PathBuf) -> Result<File> {
    let language = get_language(file_path)?;
    let text = read_to_string(file_path).map_err(|_| Error::ReadFile)?;
    Ok(File { text, language })
}
