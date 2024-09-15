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
        let link_class = "text-md sm:text-xl md:text-2xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest whitespace-nowrap";
        let (solstice_or_equinox, num_days_to_solstice_or_equinox) = crate::library::seasons::get_closest_upcoming_solstice_or_equinox();
        let solstice_or_equinox = match solstice_or_equinox.kind {
            crate::library::seasons::SunStationKind::MarchEquinox => "Equinox",
            crate::library::seasons::SunStationKind::JuneSolstice => "Solstice",
            crate::library::seasons::SunStationKind::SeptemberEquinox => "Equinox",
            crate::library::seasons::SunStationKind::DecemberSolstice => "Solstice",
        };
        let right_text = format!("{solstice_or_equinox}: {num_days_to_solstice_or_equinox} days");
        html! {
            nav class={"w-full pb-4 md:pb-8 xl:pb-12 " (self.class)} {
                div class="name flex flex-row justify-between" {
                    (Link::new()
                        .href(Route::Home)
                        .without_default_classes()
                        .class(link_class)
                        .slot("Luca Aurelia")
                    )
                    p class=(link_class) {
                        (right_text)
                    }
                }
            }
        }
    }
}
