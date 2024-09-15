use crate::obsidian::Vault;
use crate::sunful;

pub fn test_sunful() {
    println!("Testing Sunful parser on 2024 haiku.");

    println!("Loading production vault.");
    let vault = Vault::production_vault();

    println!("Finding Haiku 2024.md.");
    let haiku_2024_note = vault
        .item_at_path("Haiku 2024.md")
        .expect("Couldn't find the Haiku 2024 note.")
        .try_into_page()
        .expect("Haiku 2024 note wasn't a page.");

    let note_contents = haiku_2024_note.contents.clone();
    println!("Parsing the doc with Sunful.");
    let sunful_document = sunful::Document::from_string(&vault, note_contents).unwrap();
    dbg!(&sunful_document);
}

// pub fn get_embeddings_for_2024_haiku() -> Vec<Embedding> {
//     vec![Embedding {
//         name: "test_sunful",
//         code: test_sunful,
//     }]
// }
