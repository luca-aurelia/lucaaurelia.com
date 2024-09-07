use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "my eyes upon the sun upon my face",
    id: WorkId::MyEyesUponTheSunUponMyFace,
    year: Year::new(2023),
    month: Month::new(07),
    kind: Kind::Art,
    palette: [
        "rgb(178, 199, 199)",
        "rgb(244, 245, 237)",
        "rgb(239, 237, 194)",
    ],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/my eyes upon the sun upon my face preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/my eyes upon the sun upon my face.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Twists of light blue dots on a cream background. 2023. Luca Aurelia.";
    let haiku = [
        "the fireflies that it rained all day",
        "grind up clouds into muddy ground",
        "at last the year's first bugbite",
    ];
    let name_slot = html! {
        "my eyes upon"
        br;
        "the sun upon"
        br;
        "my face"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
