use assets::*;
use once_cell::sync::Lazy;

pub mod processed_2023_haiku;

pub static ASSETS: Lazy<Assets> = Lazy::new(Assets::new);

pub struct Assets {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub favicon: FileAsset,
    pub mask_favicon: FileAsset,
    pub apple_touch_icon: FileAsset,
    pub google_touch_icon: FileAsset,
    pub manifest_json: FileAsset,
    pub hero_image: LightDarkImageAsset,
}

impl Assets {
    pub fn new() -> Self {
        // assets::include_public_obsidian_pages!();

        // The resulting Tailwind file is saved in
        // both the built assets folder and Obsidian's
        // CSS snippets folder.
        //
        // If you change the file name in the URL path,
        // make sure to also update Obsidian to use the
        // new file name when loading the CSS snippet.
        //
        // As of April 1st, 2024, you can find that here:
        // Obsidian → Settings → Appearance → CSS snippets (at the bottom)
        let css = assets::include_tailwind!(
            path_to_input_file: "server/src/assets/main.css",
            url_path: "built-assets/built.css",
            performance_budget_millis: 150,
        );

        let browser_crate = assets::include_browser_crate!(
            path_to_browser_crate: "browser",
            js_url_path: "built-assets/browser.js",
            js_performance_budget_millis: 150,
            wasm_url_path: "built-assets/browser_bg.wasm",
            wasm_performance_budget_millis: 310,
            production: true,
        );

        let favicon = assets::include_file!(
            path_to_input_file: "server/src/assets/images/favicon.svg",
            url_path: "built-assets/favicon.svg",
            performance_budget_millis: 150,
        );

        let mask_favicon = assets::include_file!(
            path_to_input_file: "server/src/assets/images/mask_favicon.svg",
            url_path: "built-assets/mask-favicon.svg",
            performance_budget_millis: 150,
        );

        let apple_touch_icon = assets::include_file!(
            path_to_input_file: "server/src/assets/images/apple_touch_icon.png",
            url_path: "built-assets/apple-touch-icon.png",
            performance_budget_millis: 150,
        );

        let google_touch_icon = assets::include_file!(
            path_to_input_file: "server/src/assets/images/google_touch_icon.png",
            url_path: "built-assets/google-touch-icon.png",
            performance_budget_millis: 150,
        );

        let manifest_json = assets::include_file!(
            path_to_input_file: "server/src/assets/manifest.json",
            url_path: "built-assets/manifest.json",
            performance_budget_millis: 150,
        );

        // This image is decorative, so we skip the alt text.
        let hero_image_light = assets::include_image!(
            path_to_image: "server/src/assets/images/hasui_light.jpeg",
            alt: "",
            placeholder: automatic_color,
        );

        // This image is decorative, so we skip the alt text.
        let hero_image_dark = assets::include_image!(
            path_to_image: "server/src/assets/images/hasui_dark.jpeg",
            alt: "",
            placeholder: automatic_color,
        );

        let hero_image = LightDarkImageAsset::new(hero_image_light, hero_image_dark);

        Self {
            css,
            browser_crate,
            favicon,
            mask_favicon,
            apple_touch_icon,
            google_touch_icon,
            manifest_json,
            hero_image,
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}
