use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "walking the grounds of kōya mountain",
    id: WorkId::WalkingTheGroundsOfKoyaMountain,
    year: Year::new(2021),
    month: Month::new(09),
    kind: Kind::Art,
    palette: ["rgb(25, 27, 20)",  "rgb(120, 119, 115)",  "rgb(201, 200, 196)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/walking the grounds of kōya mountain preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/walking the grounds of kōya mountain.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "A triangulated, noisy face. 2021. Luca Aurelia.";
    let haiku = [
        "the sweat that squeezed a walk out of me",
        "unhindered through the tree colony",
        "comfiest death?",
    ];
    let name_slot = html! {
        "walking the"
        br;
        "grounds of"
        br;
        "kōya mountain"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}




