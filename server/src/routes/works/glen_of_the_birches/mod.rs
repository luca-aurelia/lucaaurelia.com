use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "glen of the birches",
    id: WorkId::GlenOfTheBirches,
    year: Year::new(2021),
    month: Month::new(11),
    kind: Kind::Art,
    accent_color: "rgb(127, 97, 53)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/glen of the birches preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/glen of the birches.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "glen of the birches"
        }
    }
}
