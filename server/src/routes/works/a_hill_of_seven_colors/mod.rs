use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "a hill of seven colors",
    id: WorkId::AHillOfSevenColors,
    year: Year::new(2021),
    month: Month::new(06),
    kind: Kind::Art,
    palette: ["rgb(178, 104, 110)", "rgb(178, 104, 110)", "rgb(178, 104, 110)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/a hill of seven colors preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/a hill of seven colors.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "a hill of seven colors"
        }
    }
}
