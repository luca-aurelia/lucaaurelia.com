use maud::{html, Markup, Render};

pub struct HeroSection {
    pub slot: Markup,
    pub class: String,
}

impl HeroSection {
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

impl Render for HeroSection {
    fn render(&self) -> Markup {
        html! {
            section
                class={"
                    hero
                    h-dvh
                    flex flex-col
                    justify-center items-center "
                    (self.class)} {
                (self.slot)
            }
        }
    }
}
