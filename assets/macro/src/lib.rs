use proc_macro::TokenStream;

mod browser_crate;
mod cache;
mod file;
mod file_change;
mod font;
mod images;
mod logger;
mod parse_macro_arguments;
mod public_obsidian_pages;
mod tailwind;

#[proc_macro]
pub fn include_tailwind(input: TokenStream) -> TokenStream {
    tailwind::include(input)
}

#[proc_macro]
pub fn include_browser_crate(input: TokenStream) -> TokenStream {
    browser_crate::include(input)
}

#[proc_macro]
pub fn include_image(input: TokenStream) -> TokenStream {
    images::include(input)
}

#[proc_macro]
pub fn include_font(input: TokenStream) -> TokenStream {
    font::include(input)
}

#[proc_macro]
pub fn include_file(input: TokenStream) -> TokenStream {
    file::include(input)
}

#[proc_macro]
pub fn include_public_obsidian_pages(input: TokenStream) -> TokenStream {
    public_obsidian_pages::include(input)
}

// #[proc_macro]
// pub fn include_images(input: TokenStream) -> TokenStream {
//     lock_file::with_lock_file("include_images", || images::include_images_in_folder(input))
// }
