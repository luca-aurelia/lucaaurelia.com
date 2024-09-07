use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "the sun not setting turned to moss",
    id: WorkId::TheSunNotSettingTurnedToMoss,
    year: Year::new(2022),
    month: Month::new(11),
    kind: Kind::Art,
    palette: [
        "rgb(96, 113, 99)",
        "rgb(130, 151, 127)",
        "rgb(187, 191, 173)",
    ],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/the sun not setting turned to moss preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/the sun not setting turned to moss.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Islands of moss on imaginary tiles. 2022. Luca Aurelia.";
    let haiku = [
        "out on the hillside, foraging words for yellow",
        "up here the cloudwater gazing site",
        "shoulder to shoulder every green",
    ];
    let name_slot = html! {
        "the sun not"
        br;
        "setting turned"
        br;
        "to moss"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}
