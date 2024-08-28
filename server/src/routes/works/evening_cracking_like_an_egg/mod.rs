use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "evening cracking like an egg",
    id: WorkId::EveningCrackingLikeAnEgg,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    accent_color: "rgb(92, 42, 29)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/evening cracking like an egg preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/evening cracking like an egg.png",
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
