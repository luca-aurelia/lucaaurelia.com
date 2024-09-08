use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "green like no grass is green",
    id: WorkId::GreenLikeNoGrassIsGreen,
    year: Year::new(2022),
    month: Month::new(06),
    kind: Kind::Art,
    palette: ["rgb(101, 142, 92)", "rgb(191, 217, 187)", "rgb(168, 198, 224)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/green like no grass is green preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/green like no grass is green.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Simulated rolling hills. 2022. Luca Aurelia.";
    let haiku = [
        "the air where the butterflies",
        "no goats here but there’s a bird trying her best",
        "the plants who season water with sunlight",
    ];
    let name_slot = html! {
        "green like"
        br;
        "no grass is green"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}


