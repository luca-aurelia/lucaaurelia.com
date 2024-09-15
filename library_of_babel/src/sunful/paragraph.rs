use super::line::Line;
use super::section_metadata::SectionMetadata;

#[derive(Debug)]
pub struct Paragraph {
    pub lines: Vec<Line>,
    /// Each paragraph owns its own metadata because paragraphs
    /// can overwrite metadata from previous paragraphs.
    pub metadata: SectionMetadata,
}

impl Paragraph {
    pub fn new(metadata: SectionMetadata, lines: Vec<Line>) -> Self {
        Paragraph { lines, metadata }
    }
}
