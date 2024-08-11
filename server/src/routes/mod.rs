use crate::assets::processed_2023_haiku;
use crate::components::{Image, Layout, Link, Logo};
use crate::css_class_groups::bg_background;
use crate::extensions::VecExtension;
use crate::library::seasons::{get_closest_upcoming_solstice_or_equinox, SunStationKind};
use crate::library::work_index::WORK_INDEX;
use maud::{html, Markup, PreEscaped, Render};
use number_to_words::number_to_words;
use shared::route::Route;

mod build_time;
mod not_found;
pub mod robots_txt;
pub mod route;
mod santoka;
mod work;

pub fn page() -> Markup {
    let layout = Layout::new(
        "Luca Aurelia — Writer + Generative Artist",
        "The home page of artist and writer Luca Aurelia — creative coding, music, and scraps of poems like little polaroids made of words.",
        html! {
            main
                class="w-full p-4 lg:p-8 flex flex-row" {
                div class="about-container md:w-[460px] lg:w-[512px] xl:w-[680px] shrink-0 flex flex-col fixed left-4 md:left-8 top-4 md:top-8 bottom-4 md:bottom-8 pr-4 md:pr-8 z-10" {
                    div class="about w-full h-full flex flex-col gap-10 justify-center items-center" {
                        div class="about-text flex flex-col" {
                            div class="name" {
                                div class="colors flex flex-row overflow-hidden rounded-[0.0625rem]" {
                                    div class="w-4 h-4" style="background-color: rgb(243, 133, 138);" {}
                                    div class="w-4 h-4" style="background-color: rgb(245, 161, 183);" {}
                                    div class="w-4 h-4" style="background-color: rgb(255, 206, 189);" {}
                                }
                                div class="spacer w-full h-4" {}
                                h1 class="text-4xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest" {
                                    "Luca Aurelia"
                                }
                            }
                            div class="spacer w-full h-10" {}
                            div class="details" {
                                p class="text-2xl font-extralight text-neutral-600 dark:text-neutral-300 tracking-wide" {
                                    "1990 — 20XX"
                                }
                                div class="spacer w-full h-4" {}
                                p class="text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                                    "trees tongue the sunny morning"
                                }

                            }
                        }
                    }
                }
                div class="about-placeholder md:w-[460px] lg:w-[512px] xl:w-[680px] shrink-0 h-full" {}
                ol class="works
                    group/ol
                    w-3/5 shrink
                    flex flex-col
                    text-2xl/[1.25]
                    tracking-wide font-light
                    " {
                    @for work in WORK_INDEX.works() {
                        li class="work-link
                            external:show-preview-on-hover
                            group/li
                            w-full
                            relative
                            -mb-px
                            external:container-for-delay-z-index-transition-on-unhover
                            select-none
                            xl:pr-10
                            " {
                            // Fake border top and bottom that, unlike border, don't actually affect layout.
                            span class="fake-border-top
                                z-10 group-hover/li:z-20
                                absolute inset-x-0 top-0 h-px
                                bg-neutral-500 group-hover/li:!bg-neutral-500 group-hover/ol:bg-neutral-300
                                dark:bg-neutral-300 group-hover/li:dark:!bg-neutral-300 group-hover/ol:dark:bg-neutral-600
                                "
                                {}
                            span class="fake-border-bottom
                                z-10 group-hover/li:z-20
                                external:delay-z-index-transition-on-unhover
                                absolute inset-x-0 bottom-0 h-px
                                bg-neutral-500 group-hover/li:!bg-neutral-500 group-hover/ol:bg-neutral-300
                                dark:bg-neutral-300 group-hover/li:dark:!bg-neutral-300 group-hover/ol:dark:bg-neutral-600
                                "
                                {}

                            a href="https://google.com" class="py-4 block w-full flex flex-row gap-6 items-center" {
                                span class="work-date
                                    hidden xl:block
                                    text-neutral-400 group-hover/li:!text-neutral-500 group-hover/ol:text-neutral-200
                                    dark:text-neutral-400 group-hover/li:dark:!text-neutral-300 group-hover/ol:dark:text-neutral-700
                                    tabular-nums text-base
                                    translate-y-[0.175rem]
                                    transition-colors duration-200 ease-out
                                    whitespace-nowrap
                                    " {
                                    (work.year) " · " (work.month)
                                }
                                span class="work-name block
                                    text-neutral-600 group-hover/li:!text-neutral-700 group-hover/ol:text-neutral-300
                                    dark:text-neutral-200 group-hover/li:dark:!text-neutral-100 group-hover/ol:dark:text-neutral-500
                                    transition-colors duration-200 ease-out
                                    " {
                                    (work.name)
                                }
                                div class="hover-indicator
                                    hidden xl:block
                                    absolute right-2
                                    rounded-[0.125rem]
                                    h-6 w-6
                                    transition duration-200 ease-out
                                    -translate-x-1 group-hover/li:translate-x-0
                                    opacity-0 group-hover/li:opacity-100
                                    "
                                    style={"background-color: " (work.accent_color) ";" } {}
                            }
                        }
                        div class="external:preview-container fixed left-4 lg:left-8 top-4 lg:top-8 bottom-4 lg:bottom-8 md:w-[460px] lg:w-[512px] xl:w-[680px] pr-4 lg:pr-8 overflow-hidden flex flex-col items-center justify-center" {
                            div class={ "external:preview-backdrop opacity-0 absolute left-0 top-0 w-full h-full " (bg_background()) } {}
                            (Image::new(&work.preview_image)
                                .class("external:preview-image opacity-0 transition-opacity duration-0 w-full h-full block object-cover object-center rounded-md z-10"))
                        }
                    }
                }
            }
        },
    );

    layout.render()
}
