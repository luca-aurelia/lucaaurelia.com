use assets::{BrowserCrateAsset, CssAsset, FileAsset, ImageAsset};
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use once_cell::sync::Lazy;

use crate::components::*;
use crate::css_class_groups::*;

struct AssetIndex {
    pub css: CssAsset,
    pub browser_crate: BrowserCrateAsset,
    pub favicon: FileAsset,
    pub mask_favicon: FileAsset,
    pub apple_touch_icon: FileAsset,
    pub google_touch_icon: FileAsset,
    pub manifest_json: FileAsset,
    pub open_graph_image: ImageAsset,
}

static ASSET_INDEX: Lazy<AssetIndex> = Lazy::new(|| AssetIndex {
    css: assets::include_tailwind!(
        path_to_input_file: "server/src/assets/main.css",
        url_path: "/built-assets/built.css",
        performance_budget_millis: 160,
    ),
    browser_crate: assets::include_browser_crate!(
        path_to_browser_crate_from_workspace_root: "browser",
        js_url_path: "/built-assets/browser.js",
        js_performance_budget_millis: 150,
        wasm_url_path: "/built-assets/browser_bg.wasm",
        wasm_performance_budget_millis: 310,
        production: true,
    ),
    favicon: assets::include_file!(
        path_to_input_file: "server/src/assets/images/favicon.svg",
        url_path: "/built-assets/favicon.svg",
        performance_budget_millis: 150,
    ),
    mask_favicon: assets::include_file!(
        path_to_input_file: "server/src/assets/images/mask favicon.svg",
        url_path: "/built-assets/mask-favicon.svg",
        performance_budget_millis: 150,
    ),
    apple_touch_icon: assets::include_file!(
        path_to_input_file: "server/src/assets/images/apple touch icon.png",
        url_path: "/built-assets/apple-touch-icon.png",
        performance_budget_millis: 150,
    ),
    google_touch_icon: assets::include_file!(
        path_to_input_file: "server/src/assets/images/google touch icon.png",
        url_path: "/built-assets/google-touch-icon.png",
        performance_budget_millis: 150,
    ),
    manifest_json: assets::include_file!(
        path_to_input_file: "server/src/assets/manifest.json",
        url_path: "/built-assets/manifest.json",
        performance_budget_millis: 150,
    ),
    open_graph_image: assets::include_image!(
        path_to_image: "server/src/assets/images/evening cracking like an egg.png",
        alt: "evening cracking like an egg",
        placeholder: automatic_color,
    ),
});

pub struct Layout {
    title: &'static str,
    description: &'static str,
    open_graph_image: &'static ImageAsset,
    use_default_body_classes: bool,
    selection_classes: String,
    slot: Markup,
}

impl Layout {
    pub fn new(title: &'static str, description: &'static str, slot: Markup) -> Self {
        Self {
            title,
            description,
            open_graph_image: &ASSET_INDEX.open_graph_image,
            use_default_body_classes: true,
            selection_classes: "selection:bg-neutral-500 selection:text-neutral-100 dark:selection:bg-neutral-300 dark:selection:text-neutral-700".to_string(),
            slot,
        }
    }

    pub fn open_graph_image(mut self, open_graph_image: &'static ImageAsset) -> Self {
        self.open_graph_image = open_graph_image;
        self
    }

    pub fn use_default_body_classes(mut self, default_body_classes: bool) -> Self {
        self.use_default_body_classes = default_body_classes;
        self
    }

    pub fn selection_classes(mut self, selection_classes: String) -> Self {
        self.selection_classes = selection_classes;
        self
    }
}

impl Render for Layout {
    fn render(&self) -> Markup {
        //
        let body_classes = if self.use_default_body_classes {
            let bg_background = bg_background();
            let selection_classes = &self.selection_classes;
            format!("flex flex-col items-center {bg_background} {selection_classes}")
        } else {
            String::new()
        };

        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http_equiv="X-UA-Compatible" content="ie=edge";

                    link rel="icon" href=(ASSET_INDEX.favicon.url_path.to_string_lossy().to_string());
                    link rel="mask-icon" href=(ASSET_INDEX.mask_favicon.url_path.to_string_lossy().to_string()) color="#232323";
                    link rel="apple-touch-icon" href=(ASSET_INDEX.apple_touch_icon.url_path.to_string_lossy().to_string());
                    link rel="manifest" href=(ASSET_INDEX.manifest_json.url_path.to_string_lossy().to_string());
                    meta name="theme-color" content="#232323";

                    (stylesheet(&ASSET_INDEX.css))

                    title {
                        (self.title)
                    }
                    meta name="description" content=(self.description);
                    meta property="og:site_name" content="Luca Aurelia";
                    meta property="og:url" content="https://lucaaurelia.com/";
                    meta property="og:type" content="article";
                    meta property="og:title" content=(self.title);
                    meta property="og:type" content=(self.description);
                    meta property="og:image" content=(self.open_graph_image.medium_sized_full_url);
                }

                body
                    class=(body_classes) {
                    (self.slot)
                    (main_js())
                }
            }
        }
    }
}

fn main_js() -> Markup {
    let browser_js_path = ASSET_INDEX
        .browser_crate
        .js
        .url_path
        .to_string_lossy()
        .to_string();
    let contents =
        include_str!("../assets/main.js").replace("{browser_js_filename}", &browser_js_path);
    html! {
        script type="module" {
            (PreEscaped(contents))
        }
    }
}
