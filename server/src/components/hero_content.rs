use maud::{html, Markup, Render};

pub struct HeroContent {
    pub slot: Markup,
    pub class: String,
}

impl HeroContent {
    pub fn new() -> Self {
        Self {
            slot: html! {},
            class: "".to_string(),
        }
    }

    pub fn slot(mut self, slot: Markup) -> Self {
        self.slot = slot;
        self
    }

    #[allow(dead_code)]
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for HeroContent {
    fn render(&self) -> Markup {
        html! {
            div class={"hero-content w-[80vw] h-[80dvh] rounded " (self.class)} {
                (self.slot)
            }
        }
    }
}
