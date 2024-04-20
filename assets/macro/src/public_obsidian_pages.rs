use crate::parse_macro_arguments::*;
use paths::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Result as SynResult,
};

pub fn include(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as Input);
    crate::logger::init_logger(input.debug);

    let vault = library_of_babel::obsidian::Vault::production_vault();
    let public_pages = vault.compile_checked_pages.public_page.back_links(&vault);

    std::fs::create_dir_all(built_assets_dir()).expect("Error creating built assets dir.");

    for public_page in public_pages {
        // Copy public_page into built_assets_dir.
        let input_path = &public_page.file.absolute_path;
        let output_path = built_assets_dir().join(&public_page.file.path_from_vault_root);
        let output_dir = output_path
            .parent()
            .expect("Error getting parent dir for public Obsidian page.");
        std::fs::create_dir_all(output_dir).expect("Error creating path for public Obsidian page.");

        // Copying fails if the file already exists, so try to delete the file.
        //
        // It's fine if deleting the file fails. The most common reason is that the file
        // doesn't exist yet.
        let _error = std::fs::remove_file(&output_path);
        std::fs::copy(input_path, output_path).expect("Error copying public Obsidian page.");

        // Copy all embeds into built_assets_dir.
        let embedded_files = public_page.embedded_files(&vault);
        for embedded_file in embedded_files {
            let input_path = &embedded_file.absolute_path;
            let output_path = built_assets_dir().join(&embedded_file.path_from_vault_root);
            let output_dir = output_path
                .parent()
                .expect("Error getting parent dir for file embedded in public Obsidian page.");
            std::fs::create_dir_all(&output_dir)
                .expect("Error creating path for file embedded in public Obsidian page.");
            std::fs::copy(input_path, output_path)
                .expect("Error copying file embedded in public Obsidian page.");
        }
    }

    quote!().into()
}

struct Input {
    debug: bool,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let debug = parse_named_bool_argument("debug", &input).unwrap_or(false);
        Ok(Input { debug })
    }
}
