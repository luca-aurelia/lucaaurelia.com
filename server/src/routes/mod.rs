use crate::components::{ColorBlocks, Image, Layout, Link, Nav};
use crate::css_class_groups::bg_background;
use crate::library::work::Work;
use crate::library::breakpoint::Breakpoint;
use maud::{html, Markup, Render};
use shared::route::Route;

mod build_time;
mod not_found;
pub mod robots_txt;
pub mod route;
pub mod santoka;
pub mod works;

pub fn page() -> Markup {
    let layout = Layout::new(
        "Luca Aurelia — Writer + Generative Artist",
        "The home page of artist and writer Luca Aurelia — creative coding, music, and scraps of poems like little polaroids made of words.",
        page_html(),
    );

    layout.render()
}

fn page_html() -> Markup {
    html! {
        main
            class="w-full p-4 md:p-8 lg:p-12 flex flex-col lg:flex-row" {
            (about())
            (works())
        }
    }
}

fn about() -> Markup {
    let most_light_speaks_sunish = &crate::routes::works::most_light_speaks_sunish::WORK;
    html! {
        (Nav::new().class("block lg:hidden"))
        div class="about-placeholder hidden lg:block lg:w-[680px] shrink-0 h-full" {}
        div class="
            about-container
            w-full lg:w-[680px]
            flex flex-col
            lg:fixed md:left-8 lg:left-12 lg:top-12 lg:bottom-12 lg:pr-12
            pb-6 sm:pb-8 md:pb-12 lg:pb-0
            z-10
            " {
            div class="about w-full h-full flex flex-col justify-center items-start lg:items-start" {
                div class="about-text hidden lg:flex flex-col" {
                    div class="name flex flex-col items-start" {
                        (ColorBlocks::new(&most_light_speaks_sunish.palette).size_breakpoint(Breakpoint::Md))
                        div class="spacer w-full h-4 shrink-0" {}
                        h1 class="text-2xl lg:text-6xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest" {
                            "Luca Aurelia"
                        }
                        div class="spacer w-full h-2 h-8 shrink-0" {}
                        h1 class="text-6xl uppercase font-extralight text-neutral-700 dark:text-neutral-100 tracking-widest whitespace-nowrap" {
                            span {
                                "Haiku "
                            }
                            span class="inline-block -translate-y-[4px]" {
                                "+"
                            }
                            br;
                            span {
                                "Generative Art"
                            }
                        }
                    }
                    div class="spacer hidden //-lg:block w-full h-6 lg:h-10" {}
                    div class="details hidden //-lg:block" {
                        p class="text-base/[1.25] lg:text-2xl font-extralight text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            "1990 — 20XX"
                        }
                        div class="spacer w-full h-1 lg:h-4" {}
                        p class="text-base/[1.25] lg:text-2xl italic lowercase font-extralight -translate-y-[0.25rem] text-neutral-600 dark:text-neutral-300 tracking-wide" {
                            "trees tongue the sunny morning"
                        }

                    }
                }
                div class="mobile-image lg:hidden" {
                    div class="image-size-constrainer sm:max-h-[384px] rounded-md overflow-hidden" {
                        // We scale this up to 125% since the right side has a small white edge that makes
                        // the image look uneven at 100%.
                        (Image::new(&most_light_speaks_sunish.image)
                            .class("w-full block object-cover object-center scale-125"))
                    }
                }
            }
        }
    }
}

fn works() -> Markup {
    html! {
        ol class="works
            group/ol
            w-full
            flex flex-col
            tracking-wide font-light
            " {
            @for work in Work::all() {
                (work_li(work))
            }
        }
    }
}

fn work_li(work: &Work) -> Markup {
    html! {
        li class="work-link
            external:show-preview-on-hover
            group/li
            w-full
            relative
            -mb-px
            external:container-for-delay-z-index-transition-on-unhover
            select-none
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

            (Link::new()
                .href(Route::Work { id: work.id })
                .class("py-4 block w-full flex flex-row gap-6 items-center")
                .without_default_classes()
                .slot(html! {
                    span class="work-date
                        block
                        text-neutral-400 group-hover/li:!text-neutral-500 group-hover/ol:text-neutral-200
                        dark:text-neutral-400 group-hover/li:dark:!text-neutral-300 group-hover/ol:dark:text-neutral-700
                        tabular-nums text-xs
                        translate-y-[0.1rem] lg:translate-y-[0.175rem]
                        transition-colors duration-200 ease-out
                        whitespace-nowrap
                        " {
                        (work.year) " · " (work.month)
                    }
                    span class="work-name block
                        text-neutral-600 group-hover/li:!text-neutral-700 group-hover/ol:text-neutral-300
                        dark:text-neutral-200 group-hover/li:dark:!text-neutral-100 group-hover/ol:dark:text-neutral-500
                        text-base/[1.25] sm:text-2xl/[1.25]
                        transition-colors duration-200 ease-out
                        external:responsive-hover-indicator-padding
                        " {
                        (work.name)
                    }
                    div class="hover-indicator
                        hidden external:responsive-display-block-for-hover-indicator
                        absolute right-2
                        rounded-[0.125rem]
                        h-6 w-6
                        transition duration-200 ease-out
                        -translate-x-1 group-hover/li:translate-x-0
                        opacity-0 group-hover/li:opacity-100
                        "
                        style={"background-color: " (work.palette[0]) ";" } {}
                })
            )
        }
        div class="external:preview-container hidden lg:flex fixed left-4 lg:left-12 top-4 lg:top-12 bottom-4 lg:bottom-12 lg:w-[680px] pr-4 lg:pr-12 overflow-hidden flex-col items-center justify-center" {
            div class={ "external:preview-backdrop opacity-0 absolute left-0 top-0 w-full h-full " (bg_background()) } {}
            (Image::new(&work.cropped_preview_image)
                .class("external:preview-image w-full h-full block object-cover object-center rounded-md z-10"))
        }
    }
}
