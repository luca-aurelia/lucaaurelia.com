use std::fs;

use super::*;

#[derive(Debug, Clone)]
pub struct Page {
    pub id: VaultItemId,
    pub file: File,
    pub contents: String,
    pub link_spans: Vec<LinkSpan>,
    pub tags: Vec<String>,
}

impl Page {
    pub fn from_path(
        vault_path: &str,
        files_for_link_resolution: &[&File],
        absolute_path: &str,
        contents: String,
    ) -> Page {
        let file = File::from_path(vault_path, absolute_path);
        Page::parse(files_for_link_resolution, file, contents)
    }

    pub fn parse(files_for_link_resolution: &[&File], file: File, contents: String) -> Page {
        let parsed_page_contents = parse_page_contents(&contents, files_for_link_resolution);
        Page {
            id: VaultItemId::from_file(&file),
            file,
            contents,
            link_spans: parsed_page_contents.link_spans,
            tags: parsed_page_contents.tags,
        }
    }

    pub fn find_and_replace_text_for_links<GetNewLinkText>(
        &mut self,
        get_new_link_text: GetNewLinkText,
    ) where
        GetNewLinkText: Fn(&LinkSpan) -> String,
    {
        let mut cumulative_range_shift: i64 = 0;
        for reference_span in &mut self.link_spans {
            let new_text = get_new_link_text(reference_span);
            reference_span.shift_range(cumulative_range_shift);
            let range_shift_for_this_reference =
                reference_span.update_text(&new_text, &mut self.contents);
            cumulative_range_shift += range_shift_for_this_reference;
        }
    }

    pub fn replace_link_text(
        reference_span_to_update: &mut LinkSpan,
        new_text: String,
        link_spans: &mut Vec<LinkSpan>,
        contents: &mut String,
    ) {
        let mut cumulative_range_shift: i64 = 0;
        for reference_span in link_spans {
            reference_span.shift_range(cumulative_range_shift);
            if reference_span.range() == reference_span_to_update.range() {
                let range_shift_for_this_reference =
                    reference_span.update_text(&new_text, contents);
                cumulative_range_shift += range_shift_for_this_reference;
            }
        }
    }

    pub fn find_link_by_link_text(&self, link_text: &LinkTextStr) -> Option<&LinkSpan> {
        self.link_spans
            .iter()
            .find(|reference_span| reference_span.link_text() == link_text)
    }

    pub fn links_to(&self, target_id: &VaultItemId) -> bool {
        self.link_spans
            .iter()
            .any(|reference_span| reference_span.refers_to(target_id))
    }

    pub fn embedded_images<'vault>(&self, vault: &'vault Vault) -> Vec<&'vault File> {
        self.link_spans
            .iter()
            .filter_map(|link_span| link_span.link.embedded_image(vault))
            .collect()
    }

    pub fn embedded_files<'vault>(&self, vault: &'vault Vault) -> Vec<&'vault File> {
        self.link_spans
            .iter()
            .filter_map(|link_span| link_span.link.embedded_file(vault))
            .collect()
    }

    pub fn back_links<'this, 'vault: 'this>(
        &'this self,
        vault: &'vault Vault,
    ) -> impl Iterator<Item = &'vault Page> + 'this {
        vault.pages().filter(|page| page.links_to(&self.id))
    }

    /// Sets the contents and updates the file system.
    pub fn set_contents_and_save(&mut self, new_contents: String) {
        self.contents = new_contents;
        let parent_dir = self
            .file
            .absolute_path
            .parent()
            .expect("Error getting parent dir when updating file contents.");
        fs::create_dir_all(parent_dir)
            .expect("Error creating sub-folders for the current file's path.");
        fs::write(&self.file.absolute_path, &self.contents)
            .expect("Error writing new contents for page.");
    }
}

impl<'a> TryFrom<&'a VaultItem> for &'a Page {
    type Error = ();

    fn try_from(vault_item: &'a VaultItem) -> Result<Self, Self::Error> {
        match vault_item {
            VaultItem::Page(page) => Ok(page),
            VaultItem::NonPage { .. } => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a mut VaultItem> for &'a mut Page {
    type Error = ();

    fn try_from(vault_item: &'a mut VaultItem) -> Result<Self, Self::Error> {
        match vault_item {
            VaultItem::Page(page) => Ok(page),
            VaultItem::NonPage { .. } => Err(()),
        }
    }
}

fn parse_page_contents(page_contents: &str, files: &[&File]) -> ParsedPageContents {
    let link_spans = LinkSpan::parse_reference_spans(page_contents, files);
    let tags = Tag::parse_tags(page_contents);
    ParsedPageContents { link_spans, tags }
}

struct ParsedPageContents {
    link_spans: Vec<LinkSpan>,
    tags: Vec<String>,
}
