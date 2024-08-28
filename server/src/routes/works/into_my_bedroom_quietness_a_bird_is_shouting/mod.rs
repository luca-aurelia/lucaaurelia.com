use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "into my bedroom quietness a bird is shouting",
    id: WorkId::IntoMyBedroomQuietnessABirdIsShouting,
    year: Year::new(2023),
    month: Month::new(11),
    kind: Kind::Art,
    accent_color: "rgb(211, 157, 89)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting.png",
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
