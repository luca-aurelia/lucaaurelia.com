use crate::components::Link;
use maud::{html, Markup, Render};
use shared::route::Route;

pub struct Nav {
    class: String,
}

impl Nav {
    pub fn new() -> Self {
        Self {
            class: "".to_string(),
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for Nav {
    fn render(&self) -> Markup {
        let link_class = "text-xl md:text-2xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest whitespace-nowrap";
        html! {
            nav class={"w-full pb-4 md:pb-8 lg:pb-12 " (self.class)} {
                div class="name flex flex-row justify-between" {
                    (Link::new()
                        .href(Route::Home)
                        .without_default_classes()
                        .class(link_class)
                        .slot("Luca Aurelia")
                    )
                    (Link::new()
                        .href(Route::Home)
                        .without_default_classes()
                        .class(link_class)
                        .slot("Menu")
                    )
                }
            }
        }
    }
}
