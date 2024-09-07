use crate::components::{ColorBlocks, Image, Layout, Nav, WorkPage};
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "most light speaks sunish",
    id: WorkId::MostLightSpeaksSunish,
    year: Year::new(2024),
    month: Month::new(7),
    kind: Kind::Art,
    palette: [
        "rgb(243, 133, 138)",
        "rgb(245, 161, 183)",
        "rgb(255, 206, 189)",
    ],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/most light speaks sunish preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/most light speaks sunish.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Pink, yellow, and peach shadows on a simulated wall. 2024. Luca Aurelia.";
    let haiku = [
        "today’s the day i got done talking",
        "my knuckle’s a little bloody clouds at my feet",
        "doing my best to get tired of the sunset",
    ];
    let name_slot = html! {
        "most light"
        br;
        "speaks sunish"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
