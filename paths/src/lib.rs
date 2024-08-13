use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub fn workspace_root_dir() -> PathBuf {
    let cargo_workspace_dir = std::env!("CARGO_WORKSPACE_DIR");
    Path::new(&cargo_workspace_dir).to_path_buf()
}

pub fn assets_dir() -> PathBuf {
    workspace_root_dir().join("assets")
}

pub fn built_assets_dir() -> PathBuf {
    assets_dir().join(built_assets_dir_name())
}

pub fn output_file_path(url_path: &Path) -> PathBuf {
    let url_path_without_prefix = url_path
        .strip_prefix(built_assets_browser_prefix())
        .unwrap();

    built_assets_dir().join(url_path_without_prefix)
}

pub fn built_assets_dir_name() -> &'static str {
    "built"
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
/// For example, if you have an asset at `assets/built/built.css`,
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

pub fn production_vault_path() -> PathBuf {
    PathBuf::from_str("/Users/photon-garden/library-of-babel").unwrap()
}

pub fn development_vault_path() -> PathBuf {
    PathBuf::from_str("/Users/photon-garden/library-of-babel").unwrap()
}

pub fn css_snippets_dir() -> PathBuf {
    production_vault_path().join(".obsidian").join("snippets")
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
