use maud::{html, Markup};
use crate::library::work::{Work, Kind, WorkId};
use library_of_babel::date::{Year, Month};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "taking a breath from the night sky",
    id: WorkId::TakingABreathFromTheNightSky,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    accent_color: "rgb(51, 46, 72)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/taking a breath from the night sky preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/taking a breath from the night sky.png",
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
