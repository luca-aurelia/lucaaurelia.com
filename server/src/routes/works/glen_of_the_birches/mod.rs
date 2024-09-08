use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "glen of the birches",
    id: WorkId::GlenOfTheBirches,
    year: Year::new(2021),
    month: Month::new(11),
    kind: Kind::Art,
    palette: ["rgb(25, 26, 20)",  "rgb(127, 97, 53)",  "rgb(242, 222, 168)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/glen of the birches preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/glen of the birches.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Yellow somethings (leaves?) reach up and across a black circle. 2021. Luca Aurelia.";
    let haiku = [
        "leaves to bounce the morning into my eyes",
        "a tree’s dead yet still room and board",
        "the air far away turned yellow",
    ];
    let name_slot = html! {
        "glen of"
        br;
        "the birches"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}




