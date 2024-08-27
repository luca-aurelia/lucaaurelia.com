use once_cell::sync::Lazy;

pub static POEMS_JSON: &str = include_str!("../poems.json");
pub static PUBLICATIONS_JSON: &str = include_str!("../publications.json");
pub static TRANSLATORS_JSON: &str = include_str!("../translators.json");

mod dataset;
pub use self::dataset::*;

mod poem;
pub use self::poem::*;

mod publication;
pub use self::publication::*;

mod translator;
pub use self::translator::*;

pub static DATASET: Lazy<Dataset> = Lazy::new(Dataset::new);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dataset() {
        Dataset::new();
    }

    #[test]
    fn test_poem_foreign_keys() {
        let dataset = Dataset::new();
        for poem in &dataset.poems {
            let publication_id = poem.publication_id;

            // These functions panic if the record isn't found, failing the test.
            let publication = dataset.publication(publication_id);
            dataset.translators(&publication.translator_ids);
        }
    }

    #[test]
    fn test_publication_foreign_keys() {
        let dataset = Dataset::new();
        for publication in &dataset.publications {
            // This function panics if the record isn't found, failing the test.
            dataset.translators(&publication.translator_ids);
        }
    }
}
