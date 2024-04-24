use assets::ImageAsset;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::assets::ASSETS;
use crate::components::*;
use crate::css_class_groups::*;

pub struct Layout<'open_graph_image_url> {
    title: &'static str,
    description: &'static str,
    open_graph_image_url: &'open_graph_image_url str,
    include_body_classes: IncludeBodyClasses,
    slot: Markup,
}

impl<'open_graph_image_url> Layout<'open_graph_image_url> {
    pub fn new(
        title: &'static str,
        description: &'static str,
        open_graph_image: &'open_graph_image_url ImageAsset,
        include_body_classes: IncludeBodyClasses,
        slot: Markup,
    ) -> Self {
        Self {
            title,
            description,
            open_graph_image_url: &open_graph_image.medium_sized_full_url,
            include_body_classes,
            slot,
        }
    }
}

impl<'open_graph_image_url> Render for Layout<'open_graph_image_url> {
    fn render(&self) -> Markup {
        let body_classes = match self.include_body_classes {
            IncludeBodyClasses::Yes => format!(
                "flex flex-col items-center selection:bg-neutral-200/75 dark:selection:bg-neutral-700/75 {}",
                bg_background()
            ),
            IncludeBodyClasses::No => String::new(),
        };

        html! {
            (DOCTYPE)
            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http_equiv="X-UA-Compatible" content="ie=edge";

                    link rel="icon" href=(ASSETS.favicon.url_path.to_string_lossy().to_string());
                    link rel="mask-icon" href=(ASSETS.mask_favicon.url_path.to_string_lossy().to_string()) color="#232323";
                    link rel="apple-touch-icon" href=(ASSETS.apple_touch_icon.url_path.to_string_lossy().to_string());
                    link rel="manifest" href=(ASSETS.manifest_json.url_path.to_string_lossy().to_string());
                    meta name="theme-color" content="#232323";

                    (stylesheet(&ASSETS.css))

                    title {
                        (self.title)
                    }
                    meta name="description" content=(self.description);
                    meta property="og:site_name" content="Luca Aurelia";
                    meta property="og:url" content="https://lucaaurelia.com/";
                    meta property="og:type" content="article";
                    meta property="og:title" content=(self.title);
                    meta property="og:type" content=(self.description);
                    meta property="og:image" content=(self.open_graph_image_url);
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
    let browser_js_path = ASSETS
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

pub enum IncludeBodyClasses {
    Yes,
    No,
}
