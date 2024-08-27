use crate::*;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub poems: Vec<Poem>,
    pub translators: Vec<Translator>,
    pub publications: Vec<Publication>,
}

impl Dataset {
    // As long as our tests pass, translator and publication can't panic
    // even though they contain unwrap. That's because:
    // - Code outside this module can't construct publication ids, so all publication
    //   ids come from poems in the database.
    // - Our tests confirm that all poems have valid publication ids.
    // - We include the serialized dataset at compile time and it's immutable.

    pub fn new() -> Dataset {
        let poems = serde_json::from_str(POEMS_JSON).expect("Couldn't parse poems.json.");
        let publications =
            serde_json::from_str(PUBLICATIONS_JSON).expect("Couldn't parse publications.json.");
        let translators =
            serde_json::from_str(TRANSLATORS_JSON).expect("Couldn't parse translators.json.");
        Dataset {
            poems,
            publications,
            translators,
        }
    }

    pub fn publication(&self, id: PublicationId) -> &Publication {
        self.publications
            .iter()
            .find(|publication| publication.id == id)
            .unwrap()
    }

    pub fn translator(&self, id: TranslatorId) -> &Translator {
        self.translators
            .iter()
            .find(|translator| translator.id == id)
            .unwrap()
    }

    pub fn translators(&self, ids: &[TranslatorId]) -> Vec<&Translator> {
        ids.iter().map(|id| self.translator(*id)).collect()
    }

    pub fn publications_sorted_by_luca_ranking(&self) -> Vec<&Publication> {
        let mut publications: Vec<&Publication> = self.publications.iter().collect();
        publications.sort_by_key(|publication| publication.luca_ranking);
        publications
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self::new()
    }
}
