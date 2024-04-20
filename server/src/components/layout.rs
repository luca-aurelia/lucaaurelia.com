use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::assets::ASSETS;
use crate::components::*;
use crate::css_class_groups::*;

pub struct Layout {
    title: &'static str,
    description: &'static str,
    include_body_classes: IncludeBodyClasses,
    slot: Markup,
}

impl Layout {
    pub fn new(
        title: &'static str,
        description: &'static str,
        include_body_classes: IncludeBodyClasses,
        slot: Markup,
    ) -> Self {
        Self {
            title,
            description,
            include_body_classes,
            slot,
        }
    }
}

impl Render for Layout {
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
                    link rel="icon" href=(ASSETS.favicon.url_path.to_string_lossy().to_string());
                    link rel="mask-icon" href=(ASSETS.mask_favicon.url_path.to_string_lossy().to_string()) color="#232323";
                    link rel="apple-touch-icon" href=(ASSETS.apple_touch_icon.url_path.to_string_lossy().to_string());
                    link rel="manifest" href=(ASSETS.manifest_json.url_path.to_string_lossy().to_string());
                    meta name="theme-color" content="#232323";

                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http_equiv="X-UA-Compatible" content="ie=edge";
                    meta name="description" content=(self.description);
                    (stylesheet(&ASSETS.css))
                    title {
                        (self.title)
                    }
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
