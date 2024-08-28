use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "digging up night from the garden",
    id: WorkId::DiggingUpNightFromTheGarden,
    year: Year::new(2022),
    month: Month::new(07),
    kind: Kind::Art,
    accent_color: "rgb(198, 185, 227)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/digging up night from the garden preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/digging up night from the garden.png",
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
