use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "metal tendons of mountains, metal skins of lakes",
    id: WorkId::MetalTendonsOfMountainsMetalSkinsOfLakes,
    year: Year::new(2022),
    month: Month::new(05),
    kind: Kind::Art,
    palette: ["rgb(220, 219, 215)",  "rgb(220, 219, 215)",  "rgb(220, 219, 215)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "metal tendons of mountains, metal skins of lakes"
        }
    }
}
