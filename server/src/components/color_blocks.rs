use crate::library::breakpoint::Breakpoint;
use maud::{html, Markup, Render};

pub struct ColorBlocks {
    palette: [String; 3],
    size_breakpoint: Breakpoint,
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
            size_breakpoint: Breakpoint::Xl,
        }
    }

    pub fn size_breakpoint(mut self, size_breakpoint: Breakpoint) -> Self {
        self.size_breakpoint = size_breakpoint;
        self
    }
}

impl Render for ColorBlocks {
    fn render(&self) -> Markup {
        let small_wh = "w-3 h-3";
        let large_wh = match self.size_breakpoint {
            Breakpoint::Sm => "sm:w-4 sm:h-4",
            Breakpoint::Md => "md:w-4 md:h-4",
            Breakpoint::Lg => "lg:w-4 lg:h-4",
            Breakpoint::Xl => "xl:w-4 xl:h-4",
        };
        let block_class = format!("{small_wh} {large_wh}");

        html! {
            div class="color-blocks flex flex-row overflow-hidden rounded-[0.0625rem] shrink-0" {
                @for color in self.palette.iter() {
                    div class=(block_class) style={ "background-color: " (color) ";" } {}
                }
            }
        }
    }
}
