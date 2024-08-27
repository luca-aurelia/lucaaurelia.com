use crate::PublicationId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Poem {
    pub id: PoemId,
    pub publication_id: PublicationId,
    pub english_text: String,
    pub japanese_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct PoemId(usize);

impl Poem {
    pub fn japanese_text_is_romaji(&self) -> bool {
        match &self.japanese_text {
            Some(japanese_text) => uses_romaji(japanese_text),
            None => true,
        }
    }
}

fn uses_romaji(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii())
}
