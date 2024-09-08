use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "a hill of seven colors",
    id: WorkId::AHillOfSevenColors,
    year: Year::new(2021),
    month: Month::new(06),
    kind: Kind::Art,
    palette: ["rgb(131, 61, 51)", "rgb(178, 104, 110)", "rgb(246, 204, 221)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/a hill of seven colors preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/a hill of seven colors.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Almost like red and blue strata stacked on top of each other. 2021. Luca Aurelia.";
    let haiku = [
        "today i took dirt for a walk",
        "sent my eyes to climb the mountains",
        "feel the future and past that’s the law of the body",
    ];
    let name_slot = html! {
        "a hill of"
        br;
        "seven colors"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}



