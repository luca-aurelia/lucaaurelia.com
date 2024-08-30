use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "walking the grounds of kōya mountain",
    id: WorkId::WalkingTheGroundsOfKoyaMountain,
    year: Year::new(2021),
    month: Month::new(09),
    kind: Kind::Art,
    palette: ["rgb(120, 119, 115)",  "rgb(120, 119, 115)",  "rgb(120, 119, 115)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/walking the grounds of kōya mountain preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/walking the grounds of kōya mountain.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "walking the grounds of kōya mountain"
        }
    }
}
