use super::{File, Page};

#[derive(Debug)]
pub struct CompileCheckedPages {
    pub public_page: Page,
    pub image_garden_page: Page,
}

impl CompileCheckedPages {
    pub fn new(vault_path: &str, files_for_link_resolution: &[&File]) -> CompileCheckedPages {
        // Use include_str! to guarantee that the file exists at the expected path.
        let public_contents = ""; // include_str!("/Users/photon-garden/library-of-babel/Public.md");
        let public_absolute_path = "/Users/photon-garden/library-of-babel/Public.md";
        let public_page = Page::from_path(
            vault_path,
            files_for_link_resolution,
            public_absolute_path,
            public_contents.to_string(),
        );

        let image_garden_contents = ""; // include_str!("/Users/photon-garden/library-of-babel/Image garden.md");
        let image_garden_absolute_path = "/Users/photon-garden/library-of-babel/Image garden.md";
        let image_garden_page = Page::from_path(
            vault_path,
            files_for_link_resolution,
            image_garden_absolute_path,
            image_garden_contents.to_string(),
        );

        CompileCheckedPages {
            public_page,
            image_garden_page,
        }
    }
}
