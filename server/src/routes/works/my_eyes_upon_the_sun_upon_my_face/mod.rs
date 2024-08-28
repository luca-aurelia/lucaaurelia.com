use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "my eyes upon the sun upon my face",
    id: WorkId::MyEyesUponTheSunUponMyFace,
    year: Year::new(2023),
    month: Month::new(07),
    kind: Kind::Art,
    accent_color: "rgb(178, 199, 199)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/my eyes upon the sun upon my face preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/my eyes upon the sun upon my face.png",
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
