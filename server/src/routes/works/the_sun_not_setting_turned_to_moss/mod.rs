use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "the sun not setting turned to moss",
    id: WorkId::TheSunNotSettingTurnedToMoss,
    year: Year::new(2022),
    month: Month::new(11),
    kind: Kind::Art,
    accent_color: "rgb(130, 151, 127)",
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/the sun not setting turned to moss preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/the sun not setting turned to moss.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    html! {
        h1 {
            "the sun not setting turned to moss"
        }
    }
}
