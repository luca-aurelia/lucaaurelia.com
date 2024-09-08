use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "three dimensions of birdsong",
    id: WorkId::ThreeDimensionsOfBirdsong,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    palette: [
        "rgb(48, 52, 54)",
        "rgb(177, 107, 90)",
        "rgb(202, 143, 112)",
    ],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/three dimensions of birdsong preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/three dimensions of birdsong.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Red and blue a little like shredded cloth around a circle of yellow — maybe a coin or a sun. 2022. Luca Aurelia.";
    let haiku = [
        "today's working the wind today's resting the wind",
        "remembered i’m breathing out my body",
        "i thought again about a break in the clouds",
    ];
    let name_slot = html! {
        "three dimensions"
        br;
        "of birdsong"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
