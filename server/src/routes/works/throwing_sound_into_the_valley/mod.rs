use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "throwing sound into the valley",
    id: WorkId::ThrowingSoundIntoTheValley,
    year: Year::new(2022),
    month: Month::new(06),
    kind: Kind::Art,
    palette: ["rgb(239, 166, 173)", "rgb(211, 210, 206)", "rgb(249, 244, 238)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/throwing sound into the valley preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/throwing sound into the valley.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "A tangle of lines stacked in different colors. 2022. Luca Aurelia.";
    let haiku = [
        "tongue puddled in my mouth sounds",
        "forehearing the chickens of tomorrow morning",
        "here in a bed bird songs on repeat",
    ];
    let name_slot = html! {
        "throwing sound"
        br;
        "into the valley"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}



