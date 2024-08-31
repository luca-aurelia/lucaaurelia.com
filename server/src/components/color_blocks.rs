use maud::{html, Markup, Render};

pub struct ColorBlocks {
    palette: [String; 3],
    class: String,
}

impl ColorBlocks {
    pub fn new(palette: &[&'static str; 3]) -> Self {
        let palette_strings = [
            palette[0].to_string(),
            palette[1].to_string(),
            palette[2].to_string(),
        ];

        Self {
            palette: palette_strings,
            class: String::new(),
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = class.into();
        self
    }
}

impl Render for ColorBlocks {
    fn render(&self) -> Markup {
        let block_class = "w-3 h-3 lg:w-4 lg:h-4";

        html! {
            div class="color-blocks flex flex-row overflow-hidden rounded-[0.0625rem] shrink-0" {
                @for color in self.palette.iter() {
                    div class=(block_class) style={ "background-color: " (color) ";" } {}
                }
            }
        }
    }
}