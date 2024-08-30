use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "when sun and dirt",
    id: WorkId::WhenSunAndDirt,
    year: Year::new(2024),
    month: Month::new(5),
    kind: Kind::Art,
    palette: ["rgb(127, 157, 201)",  "rgb(127, 157, 201)",  "rgb(127, 157, 201)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/when sun and dirt preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/when sun and dirt.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "when sun and dirt"
        }
    }
}
