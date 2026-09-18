use crate::source::read::{File, Language};
use tree_sitter::{Query, QueryCursor, QueryError, StreamingIterator, Tree};

const GO_QUERY: &str = include_str!("queries/go.scm");
const PYTHON_QUERY: &str = include_str!("queries/python.scm");

#[derive(Debug)]
pub struct FoundNode {
    pub start: usize,
    pub end: usize,
    pub kind: String,
}

pub struct Finder<'a> {
    file: &'a File,
    query: Query,
    tree: &'a Tree,
}

impl<'a> Finder<'a> {
    pub fn new(file: &'a File, tree: &'a Tree) -> Result<Self, QueryError> {
        let query_source = match file.language {
            Language::Go => GO_QUERY,
            Language::Python => PYTHON_QUERY,
            //TODO: Add more languages scm
            _ => GO_QUERY,
        };

        let engine_language: tree_sitter::Language = match file.language {
            Language::Go => tree_sitter_go::LANGUAGE.into(),
            _ => tree_sitter_go::LANGUAGE.into(), //TODO: add more languages
        };

        let query = Query::new(&engine_language, query_source)?;
        Ok(Finder {
            file: file,
            query: query,
            tree: tree,
        })
    }

    pub fn find(&self) -> Vec<FoundNode> {
        let mut nodes: Vec<FoundNode> = Vec::new();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(
            &self.query,
            self.tree.root_node(),
            self.file.text.as_bytes(),
        );
        while let Some(m) = matches.next() {
            for capture in m.captures() {
                let capture_kind = self.query.capture_names()[capture.index as usize];
                let curr_node = FoundNode {
                    start: capture.node.start_byte(),
                    end: capture.node.end_byte(),
                    kind: String::from(capture_kind),
                };
                nodes.push(curr_node);
            }
        }
        nodes
    }
}
