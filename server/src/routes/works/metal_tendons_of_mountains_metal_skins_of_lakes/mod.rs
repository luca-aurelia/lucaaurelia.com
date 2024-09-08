use crate::components::WorkPage;
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "metal tendons of mountains, metal skins of lakes",
    id: WorkId::MetalTendonsOfMountainsMetalSkinsOfLakes,
    year: Year::new(2022),
    month: Month::new(05),
    kind: Kind::Art,
    palette: ["rgb(114, 113, 110)", "rgb(195, 194, 191)",  "rgb(220, 219, 215)"],
    cropped_preview_image: assets::include_image!(
        path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes preview.png",
        alt: "",
        placeholder: automatic_color,
    ),
    image: assets::include_image!(
        path_to_image: "server/src/assets/images/metal tendons of mountains, metal skins of lakes.png",
        alt: "",
        placeholder: automatic_color,
    ),
    page_fn: page,
});

pub fn page() -> Markup {
    let description = "Grays and moirés with a family resemblance to brushed steel. 2022. Luca Aurelia.";
    let haiku = [
        "here the pines are the dawn chorus",
        "light on the long ridge",
        "today the valley's a wind puddle",
    ];
    let name_slot = html! {
        "metal tendons"
        br;
        "of mountains,"
        br;
        "metal skins"
        br;
        "of lakes"
    };
    let work_page = WorkPage::new(&WORK, description, haiku).name_slot(name_slot);
    work_page.render()
}




