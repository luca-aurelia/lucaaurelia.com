use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "quiet and watching the clouds flock",
    id: WorkId::QuietAndWatchingTheCloudsFlock,
    year: Year::new(2022),
    month: Month::new(10),
    kind: Kind::Art,
    accent_color: "rgb(119, 214, 210)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/quiet and watching the clouds flock preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/quiet and watching the clouds flock.png",
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
