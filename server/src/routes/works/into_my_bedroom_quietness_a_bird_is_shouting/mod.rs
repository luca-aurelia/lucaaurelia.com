use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "into my bedroom quietness a bird is shouting",
    id: WorkId::IntoMyBedroomQuietnessABirdIsShouting,
    year: Year::new(2023),
    month: Month::new(11),
    kind: Kind::Art,
    // palette: ["rgb(198, 107, 80)",  "rgb(159, 173, 178)",  "rgb(211, 157, 89)"],
    palette: [
        "rgb(198, 107, 80)",
        "rgb(159, 173, 178)",
        "rgb(215, 197, 125)",
    ],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/into my bedroom quietness a bird is shouting.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Plotter, paper, and colored pencils. 2023. Luca Aurelia.";
    let haiku = [
        "no queen, a colony of trees",
        "who left weather vane mountain out overnight",
        "opening the canopy, dying's important work",
    ];
    let name_slot = html! {
        "into my bedroom"
        br;
        "quietness a bird"
        br;
        "is shouting"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
