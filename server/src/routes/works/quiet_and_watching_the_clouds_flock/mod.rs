
use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "quiet and watching the clouds flock",
    id: WorkId::QuietAndWatchingTheCloudsFlock,
    year: Year::new(2022),
    month: Month::new(10),
    kind: Kind::Art,
    palette: ["rgb(42, 170, 197)", "rgb(119, 214, 210)", "rgb(252, 240, 192)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/quiet and watching the clouds flock preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/quiet and watching the clouds flock.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "The clouds and sea made out of fake paint. 2022. Luca Aurelia.";
    let haiku = [
        "from here i can’t see the world curving",
        "hungering to the sound of water",
        "so far none of my bones on this beach",
    ];
    let name_slot = html! {
        "quiet and"
        br;
        "watching the"
        br;
        "clouds flock"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
