use maud::{html, Markup};
use crate::library::work::{Work, Kind, WorkId};
use library_of_babel::date::{Year, Month};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "three dimensions of birdsong",
    id: WorkId::ThreeDimensionsOfBirdsong,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    accent_color: "rgb(177, 107, 90)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/three dimensions of birdsong preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/three dimensions of birdsong.png",
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
