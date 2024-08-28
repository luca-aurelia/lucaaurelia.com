use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "most light speaks sunish",
    id: WorkId::MostLightSpeaksSunish,
    year: Year::new(2024),
    month: Month::new(7),
    kind: Kind::Art,
    accent_color: "rgb(231, 143, 129)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/most light speaks sunish preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/most light speaks sunish.png",
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
