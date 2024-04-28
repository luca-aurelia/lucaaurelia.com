use crate::components::Link;
use crate::css_class_groups::bg_background;
use crate::extensions::StrExtension;
use maud::{html, Markup, Render};
use shared::route::Route;

pub struct Logo {
    class: String,
}

impl Logo {
    pub fn new() -> Self {
        Self {
            class: "".to_string(),
        }
    }

    #[allow(unused)]
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for Logo {
    fn render(&self) -> Markup {
        Link::new()
            .href(Route::Home)
            .without_default_classes()
            .class(
                "
                logo
                flex rounded-full
                items-center justify-center
                w-10 lg:w-12 h-10 lg:h-12
                cursor-pointer

                transition
                text-neutral-300
                hover:text-neutral-500
                dark:text-neutral-600
                dark:hover:text-neutral-400 

                translate-x-px translate-y-px"
                    .join_class(bg_background())
                    .join_class(&self.class),
            )
            .slot(html! {
                p class="font-normal lg:font-normal tracking-[0.15em] text-sm font-mono" {
                    "LA"
                }
            })
            .render()
    }
}
