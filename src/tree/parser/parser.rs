use crate::error::parser::{ParserError, Result};
use crate::source::read::{File, Language};
use tree_sitter::{Parser, Tree};
use tree_sitter_go;

pub struct SourceParser<'a> {
    engine: Parser,
    source: &'a str,
}

impl<'a> SourceParser<'a> {
    pub fn new(file: &'a File) -> Result<Self> {
        let mut engine = Parser::new();

        // Add a language
        match file.language {
            Language::Go => engine
                .set_language(&tree_sitter_go::LANGUAGE.into())
                .map_err(|_| ParserError::SetLanguage)?,
            Language::Python => engine
                .set_language(&tree_sitter_python::LANGUAGE.into())
                .map_err(|_| ParserError::SetLanguage)?,
            Language::Rust => engine
                .set_language(&tree_sitter_rust::LANGUAGE.into())
                .map_err(|_| ParserError::SetLanguage)?,
            Language::Csharp => engine
                .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
                .map_err(|_| ParserError::SetLanguage)?,
        }

        Ok(SourceParser {
            engine: engine,
            source: &file.text,
        })
    }
    pub fn get_tree(&mut self) -> Result<Tree> {
        self.engine
            .parse(self.source, None)
            .ok_or(ParserError::Parse)
    }
}
