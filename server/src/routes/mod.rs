use crate::assets::{processed_2023_haiku, ASSETS};
use crate::components::{IncludeBodyClasses, Layout, Link, NavLogo};
use crate::extensions::VecExtension;
use crate::library::seasons::{get_closest_upcoming_solstice_or_equinox, SunStationKind};
use maud::{html, Markup, Render};
use number_to_words::number_to_words;
use shared::route::Route;

mod build_time;
pub mod image_garden;
mod not_found;
pub mod robots_txt;
pub mod route;
mod santoka;
mod work;

pub fn page() -> Markup {
    Layout::new(
        "Luca Aurelia — Writer + Generative Artist",
        "The home page of artist and writer Luca Aurelia — creative coding, music, and scraps of poems like little verbal polaroids.",
        &ASSETS.open_graph_image,
        IncludeBodyClasses::Yes,
        html! {
            main
                class="px-8 flex flex-col items-center justify-between h-dvh w-full" {
                div class="spacer w-full grow basis-0" {}
                (centerpiece())
                (nav())
            }
        },
    )
    .render()
}

fn centerpiece() -> Markup {
    html! {
        div class="
            centerpiece
            relative

            flex flex-col
            gap-16 md:gap-2
            md:flex-row-reverse
            items-center justify-center

            p-8
            w-[80vw] h-[80vh]

            bg-gradient-to-b
            lg:bg-gradient-to-br

            from-amber-300/20
            lg:from-amber-300/50
            to-20%
            md:to-30%

            dark:from-midnight
            //-lg:from-indigo-950
            to-20%
            md:to-30%

            border

            border-amber-300/40
            dark:border-midnight

            rounded" {
            // nav class="absolute top-0 left-0 p-4 lg:p-8" {
            //     (NavLogo::new().class("border border-amber-300 md:border-none"))
            // }
            // div class="spacer md:hidden" {}
            (glass())
            (poem())
        }
    }
}

fn poem() -> Markup {
    let today = chrono::Local::now().date_naive();
    let closest_poems = processed_2023_haiku::find_closest_poems(&today);
    let poem = closest_poems.random_element();
    let closest_sun_station = get_closest_upcoming_solstice_or_equinox();
    let days_until_next_sun_station = closest_sun_station.distance_in_days_from(today);

    let num_days = number_to_words(days_until_next_sun_station as i32, false);

    let season = match closest_sun_station.kind {
        SunStationKind::MarchEquinox => "spring / fall",
        SunStationKind::JuneSolstice => "summer / winter",
        SunStationKind::SeptemberEquinox => "spring / fall",
        SunStationKind::DecemberSolstice => "summer / winter",
    };
    let equinox_or_solstice = match closest_sun_station.kind {
        SunStationKind::MarchEquinox => "equinox",
        SunStationKind::JuneSolstice => "solstice",
        SunStationKind::SeptemberEquinox => "equinox",
        SunStationKind::DecemberSolstice => "solstice",
    };

    let name_of_sun_station = html! {
        span class="hidden xl:inline" {
            (season)
        }
        " "
        (equinox_or_solstice)
    };

    html! {
        // We force the .poem-text to line-wrap to the width
        // of the .sun-station by:
        // - Setting whitespace-nowrap on the .sun-station paragraph.
        // - Setting .w-min on the .poem div, shrinking it to the width
        //   of .sun-station.
        // - Setting .w-full on the .poem-text paragraph.
        div class="poem w-min text-xl sm:text-3xl lg:text-4xl text-left flex flex-col gap-1 md:gap-2" {
            p class="sun-station whitespace-nowrap font-extralight text-neutral-500 dark:text-neutral-300" {
                @if days_until_next_sun_station == 0 {
                    "today is the " (name_of_sun_station)
                } @else {
                    (num_days) " days to the " (name_of_sun_station)
                }
            }

            p class="poem-text md:text-right w-full font-thin italic text-neutral-400 dark:text-neutral-400" {
                (poem.text)
            }
        }
    }
}

fn glass() -> Markup {
    html! {
        div
            class={"rounded-full md:-ml-32 w-80 max-w-full aspect-square shadow-[0_4px_4px_rgb(10_10_10_/_0.25)] dark:shadow-[0_4px_4px_rgb(10_10_10_/_0.5)] shrink-0 backdrop-blur-[2px] bg-gradient-to-br from-cerulean to-cloud dark:from-twilight dark:to-neutral-900/5" } {
        }
    }
}

fn nav() -> Markup {
    html! {
        nav class="desktop-nav flex flex-row grow basis-0 text-base font-light text-neutral-400 translate-y-[3px]" {
            ul class="flex flex-row gap-4 h-full items-center" {
                li class="sm:hidden" {
                    "Hi, my name's Luca"
                }
                li class="hidden sm:list-item" {
                    "Hi, my name is Luca Aurelia"
                }
                // Add this back when I have a "work" link.
                // li class="spacer" {}
                // li {
                //     (Link::new()
                //         .href(Route::Work)
                //         .slot("work")
                //     )
                // }
                li {
                    (Link::new()
                        .href(Route::Santoka)
                        .slot("santōka")
                    )
                }
            }
        }
    }
}
