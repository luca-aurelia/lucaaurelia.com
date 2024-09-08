use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "digging up night from the garden",
    id: WorkId::DiggingUpNightFromTheGarden,
    year: Year::new(2022),
    month: Month::new(07),
    kind: Kind::Art,
    palette: ["rgb(78, 78, 98)", "rgb(198, 185, 227)", "rgb(240, 222, 149)"],
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
    let description = "Pixelated flowers through on a hazy backdrop. 2022. Luca Aurelia.";
    let haiku = [
        "against the leaves rose",
        "the sun is setting a feast of soundbirds",
        "night flowers thanks to all dirt is grave dirt ",
    ];
    let name_slot = html! {
        "digging up night"
        br;
        "from the garden"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}

