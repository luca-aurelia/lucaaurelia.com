use crate::css_class_groups::link_classes;
use crate::extensions::StrExtension;
use maud::{html, Markup, Render};
use shared::route::Route;

pub struct Link {
    class: String,
    href: Option<Route>,
    slot: Markup,
    default_classes: &'static str,
}

impl Link {
    pub fn new() -> Self {
        Self {
            class: "".to_string(),
            href: None,
            slot: html! {},
            default_classes: link_classes(),
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        let class = class.into();
        self.class = self.class.join_class(class);
        self
    }

    pub fn href(mut self, href: Route) -> Self {
        self.href = Some(href);
        self
    }

    pub fn slot(mut self, slot: impl Render) -> Self {
        self.slot = slot.render();
        self
    }

    pub fn without_default_classes(mut self) -> Self {
        self.default_classes = "";
        self
    }
}

impl Render for Link {
    fn render(&self) -> Markup {
        let class = format!(
            "{class} {default_classes}",
            class = self.class,
            default_classes = self.default_classes
        );

        match self.href {
            Some(href) => html! {
                a
                    class=(class)
                    href=(href) {
                    (self.slot)
                }
            },
            None => html! {
                button
                    class=(class) {
                    (self.slot)
                }
            },
        }
    }
}
