use maud::{html, Markup, Render};
use shared::route::Route;
use crate::components::Link;

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
        let link_class = "text-xl lg:text-2xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest whitespace-nowrap";
        html! {
            nav class={"w-full pb-4 lg:pb-8 " (self.class)} {
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