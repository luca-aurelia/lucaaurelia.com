use crate::components::{Image, Layout, Nav};
use crate::library::work::{Kind, Work, WorkId};
use library_of_babel::date::{Month, Year};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;
use crate::components::ColorBlocks;

pub static WORK: Lazy<Work> = Lazy::new(|| Work {
    name: "most light speaks sunish",
    id: WorkId::MostLightSpeaksSunish,
    year: Year::new(2024),
    month: Month::new(7),
    kind: Kind::Art,
    palette: ["rgb(243, 133, 138)", "rgb(245, 161, 183)", "rgb(255, 206, 189)"],
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
    let layout = Layout::new(
        "most light speaks sunish",
        "Simulated light on LCD, mini LED, or OLED screen. 2024. Luca Aurelia.",
        page_html(),
    );

    layout.render()
}

fn page_html() -> Markup {
    html! {
        div class="w-full h-lvh p-4 lg:p-8" {
            (Nav::new())

            main
                class="flex flex-col lg:flex-row h-[calc(100%-3rem)] lg:h-[calc(100%-4rem)]"
                {
                (about())
                (work())
            }
        }
    }
}

fn about() -> Markup {
    html! {
        div class="
            about-container
            w-full lg:w-[680px] shrink-0
            flex flex-col
            lg:pr-12
            pb-4 lg:pb-0
            z-10
            " {
            div class="about w-full h-full flex flex-col justify-center items-start lg:items-center" {
                div class="about-text flex flex-col" {
                    div class="name flex flex-row items-center lg:flex-col lg:items-start" {
                        (ColorBlocks::new(&WORK.palette))
                        div class="spacer w-4 h-full lg:w-full lg:h-2 lg:h-4" {}
                        h1 class="text-2xl lg:text-6xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest" {
                            "most light speaks sunish"
                        }
                    }
                    div class="spacer hidden lg:block w-full h-6 lg:h-6" {}
                    div class="details hidden lg:block" {
                        p class="text-sm/[1.25] lg:text-xl font-normal text-neutral-600 dark:text-neutral-300 tracking-wide tabular-nums" {
                            (WORK.year) " · " (WORK.month)
                        }
                        div class="spacer w-full h-1 lg:h-6" {}
                        p class="text-base/[1.25] lg:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            "today’s the day i got done talking"
                        }
                        div class="spacer w-full h-1 lg:h-2" {}
                        p class="text-base/[1.25] lg:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            "all dry lands are highlands"
                        }
                        div class="spacer w-full h-1 lg:h-2" {}
                        p class="text-base/[1.25] lg:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            "my knuckle’s a little bloody clouds at my feet"
                        }
                    }
                }
            }
        }
    }
}

fn work() -> Markup {
    html! {
        figure class="work
            w-full shrink
            max-h-full
            flex flex-col justify-center items-end
            tracking-wide font-light
            " {
            (Image::new(&WORK.image)
                .class("max-w-full max-h-full block rounded-md object-contain"))
        }
    }
}
