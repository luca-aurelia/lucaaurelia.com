use super::line::Line;
use super::paragraph::Paragraph;
use super::parse_error::ParseError;
use super::raw_section::RawSection;
use super::schema::Schema;
use super::section_metadata::{LineType, SectionMetadata, Valid};
use crate::obsidian::Vault;

#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    pub fn from_raw_section(
        vault: &Vault,
        schema: &Schema,
        raw_section: RawSection,
    ) -> Result<Self, ParseError> {
        let mut section_metadata = SectionMetadata::new();
        let mut paragraph_lines = vec![];
        let mut paragraphs = vec![];

        let mut save_paragraph = |section_metadata: &SectionMetadata,
                                  paragraph_lines: Vec<Line>| {
            if paragraph_lines.is_empty() {
                return;
            }

            let paragraph = Paragraph::new(section_metadata.clone(), paragraph_lines);
            paragraphs.push(paragraph);
        };

        for line in raw_section.lines {
            let line_type = SectionMetadata::line_type(vault, schema, &line);
            match line_type {
                LineType::ValidMetadataField { key, value } => {
                    section_metadata.set_field(key, value);
                }
                LineType::OtherText => match section_metadata.is_valid(schema) {
                    Valid::Yes => paragraph_lines.push(line),
                    Valid::No { missing_fields } => {
                        // Tried to add a line to the paragraph, but the metadata was invalid.
                        return Err(ParseError::missing_required_fields(line, missing_fields));
                    }
                },
                LineType::Empty => {
                    // Got an empty line. Time to start a new paragraph.
                    save_paragraph(&section_metadata, paragraph_lines);
                    paragraph_lines = vec![];
                }
            }
        }

        save_paragraph(&section_metadata, paragraph_lines);

        Ok(Section { paragraphs })
    }
}
