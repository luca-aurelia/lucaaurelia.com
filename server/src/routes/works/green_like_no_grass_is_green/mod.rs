use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "green like no grass is green",
    id: WorkId::GreenLikeNoGrassIsGreen,
    year: Year::new(2022),
    month: Month::new(06),
    kind: Kind::Art,
    accent_color: "rgb(101, 142, 92)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/green like no grass is green preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/green like no grass is green.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "most light speaks sunish"
        }
    }
}
