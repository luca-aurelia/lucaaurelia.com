use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "taking a breath from the night sky",
    id: WorkId::TakingABreathFromTheNightSky,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    palette: ["rgb(9, 10, 11)", "rgb(51, 46, 72)", "rgb(119, 143, 158)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/taking a breath from the night sky preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/taking a breath from the night sky.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "A cubical cross section of a bunch of pipes. Not a lot anti-aliasing. 2022. Luca Aurelia.";
    let haiku = [
        "ten years ago we stopped moth burning",
        "no lights tonight the only moon is the moon",
        "outside the wind blows the stars",
    ];
    let name_slot = html! {
        "taking a breath"
        br;
        "breath from"
        br;
        "the night sky"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}


