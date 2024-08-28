use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "thistle bright morning: haiku by taneda santōka",
    id: WorkId::Santoka,
    year: Year::new(2024),
    month: Month::new(5),
    kind: Kind::SpecialProjects,
    accent_color: "rgb(224, 115, 78)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/thistle bright morning preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/thistle bright morning.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "thistle bright morning: haiku by taneda santōka"
        }
    }
}
