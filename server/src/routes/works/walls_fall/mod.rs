use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "walls fall",
    id: WorkId::WallsFall,
    year: Year::new(2019),
    month: Month::new(04),
    kind: Kind::Art,
    accent_color: "rgb(239, 75, 134)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/walls fall preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/walls fall.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "walls fall"
        }
    }
}
