use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "evening cracking like an egg",
    id: WorkId::EveningCrackingLikeAnEgg,
    year: Year::new(2022),
    month: Month::new(09),
    kind: Kind::Art,
    palette: ["rgb(33, 29, 50)", "rgb(92, 42, 29)", "rgb(220, 161, 114)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/evening cracking like an egg preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/evening cracking like an egg.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Red and blue a little like shredded cloth around a circle of yellow — maybe a coin or a sun. 2022. Luca Aurelia.";
    let haiku = [
        "the hilltop red at sunset red at autumn",
        "the sound of clouds out the window",
        "sweeping the dusty sky off my porch",
    ];
    let name_slot = html! {
        "evening cracking"
        br;
        "like an egg"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
