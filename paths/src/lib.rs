mod crate_wrappers;

use camino::Utf8PathBuf;
use crate_wrappers::path::PathBufExtension;
use std::path::{Path, PathBuf};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn assets_dir() -> PathBuf {
    workspace_root_dir().join("assets")
}

pub fn built_dir() -> Utf8PathBuf {
    workspace_root_dir().into_utf8_path_buf().join("built")
}

pub fn built_assets_dir() -> PathBuf {
    built_dir().into_std_path_buf().join("built-assets")
}

pub fn built_asset_file_path(url_path: &Path) -> PathBuf {
    let url_path_without_prefix = url_path
        .strip_prefix(built_assets_browser_prefix())
        .unwrap();

    built_assets_dir().join(url_path_without_prefix)
}

pub fn assets_macros_dir() -> PathBuf {
    assets_dir().join("macro")
}

pub fn target_dir() -> PathBuf {
    workspace_root_dir().join("target")
}

/// When loading assets in the browser, URL paths should
/// start with this prefix.
///
/// For example, if you have an asset at `built/assets/built.css`,
/// then the URL path to that asset in the browser should be
/// `/built-assets/built.css`.
pub fn built_assets_browser_prefix() -> PathBuf {
    PathBuf::from("/built-assets")
}

/// For example, /built-assets/built.css
pub fn asset_url_path(sub_url_path: &Path) -> PathBuf {
    built_assets_browser_prefix().join(sub_url_path)
}

/// For example, https://lucaaurelia.com/built-assets/built.css
pub fn full_asset_url(sub_url_path: &Path) -> PathBuf {
    host()
        .join(built_assets_browser_prefix())
        .join(sub_url_path)
}

pub fn host() -> PathBuf {
    let is_production = std::env::var("RENDER").is_ok_and(|var| var == "true");
    if is_production {
        return PathBuf::from("https://lucaaurelia.com");
    }
    PathBuf::from("http://localhost:3000")
}

pub fn built_image_path(path_starting_from_images_dir: &Path) -> PathBuf {
    built_assets_dir().join(path_starting_from_images_dir)
}

pub fn production_obsidian_vault_path() -> PathBuf {
    home_dir().join("library-of-babel").into_std_path_buf()
}

pub fn development_obsidian_vault_path() -> PathBuf {
    home_dir().join("obsidian-dev").into_std_path_buf()
}

pub fn path_to_obsidian_transactions_folder(year: i16) -> Utf8PathBuf {
    production_obsidian_vault_path()
        .into_utf8_path_buf()
        .join("Money and business")
        .join(year.to_string())
        .join("Transactions")
}

pub fn css_snippets_dir() -> PathBuf {
    production_obsidian_vault_path()
        .join(".obsidian")
        .join("snippets")
}

pub fn path_to_detect_file_changes_cache(file_name: &str) -> PathBuf {
    target_dir()
        .join("detect_file_changes_cache")
        .join(file_name)
        .with_extension("json")
}

pub fn cached_macro_output_path(macro_name: &str) -> PathBuf {
    target_dir()
        .join("cached_macro_outputs")
        .join(format!("{}.rs", macro_name))
}

pub fn downloads_dir() -> Utf8PathBuf {
    home_dir().join("downloads")
}

fn home_dir() -> Utf8PathBuf {
    Utf8PathBuf::from("/Users/photon-garden")
}
