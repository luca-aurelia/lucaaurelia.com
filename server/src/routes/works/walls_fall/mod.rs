use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "walls fall",
    id: WorkId::WallsFall,
    year: Year::new(2019),
    month: Month::new(04),
    kind: Kind::Art,
    palette: ["rgb(239, 75, 134)",  "rgb(224, 195, 49)",  "rgb(11, 155, 185)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/walls fall preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/walls fall.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "A diamond chaos of colors. 2019. Luca Aurelia.";
    let haiku = [
        "stepped into the land out loud",
        "over the wallflowers spilling to collect this morning",
        "the bright sky burning potatoes",
    ];
    let name_slot = html! {
        "walls fall"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}




