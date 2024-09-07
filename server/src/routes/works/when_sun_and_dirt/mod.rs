use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;
use crate::components::WorkPage;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "when sun and dirt",
    id: WorkId::WhenSunAndDirt,
    year: Year::new(2024),
    month: Month::new(5),
    kind: Kind::Art,
    // palette: ["rgb(168, 124, 74)", "rgb(127, 157, 201)",  "rgb(217, 226, 229)"],
    palette: ["rgb(113, 146, 195)", "rgb(158, 184, 213)", "rgb(218, 225, 226)"],
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
    let description = "Twelve words on the edge of a lake. 2024. Luca Aurelia.";
    let haiku = [
        "as i’m leaving the rain",
        "the sun warms an eye a leaf",
        "the lake when i go and the lake when will i be back?",
    ];
    let name_slot = html! {
        "when sun and dirt"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}