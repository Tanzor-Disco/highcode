use crate::error::highlighter::{HighlighterError, Result};
use crate::tree::finder::finder::FoundNode;

#[derive(Debug)]
pub enum HighlightKind {
    Keyword,
    Function,
    String_,
    Comment,
}

#[derive(Debug)]
pub struct HighlightElement {
    pub start: usize,
    pub end: usize,
    pub kind: HighlightKind,
}

pub struct Highlighter<'a> {
    nodes: &'a [FoundNode],
}

impl<'a> Highlighter<'a> {
    pub fn new(nodes: &'a [FoundNode]) -> Highlighter<'a> {
        Highlighter { nodes: nodes }
    }

    pub fn highlight(&self) -> Result<Vec<HighlightElement>> {
        let mut highlights: Vec<HighlightElement> = Vec::new();
        for node in self.nodes {
            let kind = match node.kind.as_str() {
                "keyword" => HighlightKind::Keyword,
                "string" => HighlightKind::String_,
                "comment" => HighlightKind::Comment,
                "function" => HighlightKind::Function,
                _ => return Err(HighlighterError::UnknownCaptureName),
            };

            let curr_highlight = HighlightElement {
                start: node.start,
                end: node.end,
                kind: kind,
            };

            highlights.push(curr_highlight);
        }
        Ok(highlights)
    }
}
