use crate::TranslatorId;
use crate::{DATASET, Poem, Translator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    pub id: PublicationId,
    pub name: String,
    pub year: Option<u16>,
    pub translator_ids: Vec<TranslatorId>,
    pub description: String,
    pub luca_ranking: u8,
}

const NUM_PREVIEW_POEMS: usize = 3;

impl Publication {
    pub fn poems(&self) -> impl Iterator<Item = &Poem> {
        DATASET
            .poems
            .iter()
            .filter(|poem| poem.publication_id == self.id)
    }

    pub fn preview_poems(&self) -> impl Iterator<Item = &Poem> {
        self.poems().take(NUM_PREVIEW_POEMS)
    }

    pub fn non_preview_poems(&self) -> impl Iterator<Item = &Poem> {
        self.poems().skip(NUM_PREVIEW_POEMS)
    }

    pub fn year_or_unknown(&self) -> String {
        self.year
            .as_ref()
            .map_or("unknown".to_string(), |year: &u16| year.to_string())
    }

    pub fn has_non_preview_poems(&self) -> bool {
        self.non_preview_poems().next().is_some()
    }

    pub fn has_japanese_text(&self) -> bool {
        self.poems().any(|poem| poem.japanese_text.is_some())
    }

    pub fn translators(&self) -> impl Iterator<Item = &Translator> {
        self.translator_ids
            .iter()
            .map(|translator_id| DATASET.translator(*translator_id))
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct PublicationId(usize);

impl std::str::FromStr for PublicationId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(parsed) => {
                let publication_id = PublicationId(parsed);
                Ok(publication_id)
            }
            Err(e) => Err(format!("Failed to parse publication id: {}", e)),
        }
    }
}

impl std::fmt::Display for PublicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl enum_iterator::Sequence for PublicationId {
    const CARDINALITY: usize = 20;

    fn first() -> Option<Self> {
        DATASET
            .publications
            .first()
            .map(|publication| publication.id)
    }

    fn last() -> Option<Self> {
        DATASET
            .publications
            .last()
            .map(|publication| publication.id)
    }

    fn previous(&self) -> Option<Self> {
        let index = DATASET
            .publications
            .iter()
            .position(|publication| publication.id == *self)
            .expect("Failed to find publication id.");

        if index == 0 {
            return None;
        }

        DATASET
            .publications
            .get(index - 1)
            .map(|publication| publication.id)
    }

    fn next(&self) -> Option<Self> {
        let index = DATASET
            .publications
            .iter()
            .position(|publication| publication.id == *self)
            .expect("Failed to find publication id.");

        DATASET
            .publications
            .get(index + 1)
            .map(|publication| publication.id)
    }
}
