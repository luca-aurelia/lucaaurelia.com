use crate::assets::ASSETS;
use crate::components::{HeroContent, HeroSection, Image, IncludeBodyClasses, Layout};
use maud::{html, Markup, Render};

pub fn page() -> Markup {
    Layout::new(
        "Luca Aurelia | Not found",
        "Not found.",
        &ASSETS.not_found_image_two,
        IncludeBodyClasses::Yes,
        html! {
            main class="w-full" {
                (HeroSection::new()
                    .class("w-full")
                    .slot(content()))
            }
        },
    )
    .render()
}

fn content() -> Markup {
    HeroContent::new()
        .class("relative")
        .slot(html! {
            (Image::new(&ASSETS.not_found_image_two)
                .class("w-full h-full object-cover rounded overflow-hidden"))
            div
                class="
                    absolute bottom-0 left-0 w-full flex flex-col p-4 lg:p-8
                    text-neutral-100 dark:text-neutral-300 tracking-wider
                    bg-gradient-to-t from-neutral-950/30 " {
                span
                    id="name-in-kanji"
                    class="text-5xl sm:text-7xl font-semibold tracking-wide whitespace-nowrap" {
                    "Page not found"
                }
            }
            p class="
                pt-4

                absolute
                top-full
                left-1/2 transform -translate-x-1/2

                tracking-wide font-extralight text-neutral-400 dark:text-neutral-300 text-neutral-500 italic" {
                span class="" {
                    "Kawase Hasui,"
                }
                span class="" {
                    " 1933"
                }
            }
        })
        .render()
}
