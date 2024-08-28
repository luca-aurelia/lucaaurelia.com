use crate::components::Logo;
use crate::components::*;
use crate::css_class_groups::*;
use assets::LightDarkImageAsset;
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;
use shared::controllers::show_hide::ShowHide;
use shared::controllers::show_if_scrolled::show_if_scrolled;
use shared::route::Route;
use ::santoka::*;

pub mod all;
pub mod non_preview_poems;

struct AssetIndex {
    hero_image: LightDarkImageAsset,
}

static ASSET_INDEX: Lazy<AssetIndex> = Lazy::new(|| AssetIndex {
    hero_image: LightDarkImageAsset::new(
        assets::include_image!(
            path_to_image: "server/src/assets/images/hasui light.jpeg",
            alt: "",
            placeholder: automatic_color,
        ),
        assets::include_image!(
            path_to_image: "server/src/assets/images/hasui dark.jpeg",
            alt: "",
            placeholder: automatic_color,
        ),
    ),
});

pub fn page() -> Markup {
    page_with_options(InitiallyLoad::PreviewPoems)
}

#[derive(Clone, Copy)]
pub enum InitiallyLoad {
    AllPoems,
    PreviewPoems,
}

pub fn page_with_options(initial_poems_loaded: InitiallyLoad) -> Markup {
    Layout::new(
        "Taneda Santōka",
        "The free-verse haiku of Taneda Santōka, a Zen priest who wandered Japan until he died in 1940.",
        html! {
            main
                class="relative p-8 lg:p-16 !pb-40 !pt-0 flex flex-col gap-0 lg:gap-16 w-full" {
                (hero_section())
                (poetry_section(initial_poems_loaded))
            }
        },
    )
    .open_graph_image(&ASSET_INDEX.hero_image.light_mode)
    .render()
}

fn floating_nav() -> Markup {
    html! {
        div
            class={"
                cover-nav
                z-20
                absolute
                -top-32 left-0
                w-screen
                p-32 "
                (bg_background())} {}
        nav
            class={"
                fixed top-0 z-10
                {horizontal_center_fixed()}
                w-full max-w-screen-2xl
                pointer-events-none
                px-8 lg:px-8
                text-base lg:text-2xl tracking-wide "
                (show_if_scrolled())} {

            div
                // We use this extra div to prevent text from showing up once it has scrolled
                // up past the nav.
                class={"
                    vertical-spacer
                    pt-4 lg:pt-8
                    lg:w-1/4
                    lg:rounded-b-3xl
                    shadow-neutral-50 dark:shadow-neutral-900 "
                    (bg_background())
                }
                style="box-shadow: var(--tw-shadow-color) 0px 0px 0 2px;" {

                div
                    class="
                        logo-and-links
                        bg-[linear-gradient(90deg,_#e6edee,_#98b7ca)]
                        dark:bg-[linear-gradient(90deg,#5b6f76,#212123)]
                        overflow-hidden rounded-3xl
                        p-4
                        w-full
                        flex flex-row gap-4 justify-between
                        pointer-events-auto
                        select-none" {
                    (Logo::new())
                    // (nav_links())
                }
            }
        }
    }
}

pub fn nav_links() -> Markup {
    html! {
       div
           class="
                links
                flex flex-row gap-4
                font-light lg:font-normal
                text-neutral-100 dark:text-neutral-400" {
        //    (Link::new()
        //        .href(Route::Home)
        //        .class("tracking-wide")
        //        .slot("about"))
           (Link::new()
               .href(Route::Home)
               .class("tracking-wide")
               .slot("load all"))
       }
    }
}

fn hero_section() -> Markup {
    html! {
        section
            class="h-dvh flex flex-col justify-center items-center" {
            div
                id="hero"
                class="
                        hero-image-container
                        flex flex-col justify-start items-start dark:justify-end
                        overflow-hidden
                        rounded
                        relative
                        w-[80vw] h-[80dvh]
                        selection:bg-neutral-700/75 dark:selection:bg-neutral-500/75
                        text-neutral-100 dark:text-neutral-300 tracking-wider
                        z-20" {
                nav
                    class="
                            logo-and-links absolute p-4 lg:p-8 top-0 left-0 w-full
                            flex flex-row gap-4 justify-between
                            text-base lg:text-2xl tracking-wider text-neutral-100 dark:text-neutral-200
                            z-10" {
                    (Logo::new())
                }

                (hero_image())

                div
                    class="
                        absolute bottom-0 left-0 w-full flex flex-col p-4 lg:p-8
                        bg-gradient-to-t dark:bg-none from-neutral-950/30 " {
                    span
                        id="name-in-kanji"
                        class="text-5xl sm:text-7xl font-semibold tracking-wide whitespace-nowrap" {
                        "種田山頭火"
                    }
                    span
                        id="name-in-romaji"
                        class="text-2xl sm:text-4xl font-normal tracking-wide whitespace-nowrap" {
                        "Taneda Santōka"
                    }
                    span
                        id="birth-and-death"
                        class="text-base sm:text-3xl font-normal tracking-wide whitespace-nowrap" {
                        "1882 – 1940"
                    }
                }
            }
        }
    }
}

fn poetry_section(initial_poems_loaded: InitiallyLoad) -> Markup {
    // The idea behind this padding-top calculation is:
    //
    // The desired space between the hero image and this
    // element is 10rem, since that's how much space is
    // between each individual publication.
    //
    // The hero image is 80dvh, centered in the viewport,
    // meaning there will be 10dvh of space at the bottom.
    //
    // Our goal is:
    // padding-top + 10dvh = 10rem
    //
    // With a little algebra, this gives us:
    // padding-top = 10rem - 10dvh
    html!(
        section id="poems" class="flex flex-col tracking-wide items-center gap-40 pt-[calc(10rem-10dvh)]" {
            @for publication in DATASET.publications_sorted_by_luca_ranking() {
                (poems_and_publication(publication, initial_poems_loaded))
            }
        }
    )
}

fn poems_and_publication(
    publication: &'static Publication,
    initial_poems_loaded: InitiallyLoad,
) -> Markup {
    let show_hide = ShowHide::new();

    html!(
        div
            class={ "poems-and-publication flex flex-col gap-10 lg:gap-8 w-[48rem] max-w-full " (show_hide.container())} {
            (publication_container(publication, &show_hide))
            (poems_in_publication(publication, &show_hide, initial_poems_loaded))
        }
    )
}

fn publication_container(publication: &'static Publication, show_hide: &ShowHide) -> Markup {
    let translator_names: Vec<_> = publication.translators().map(|translator| translator.name.as_str()).collect();

    html!(
        div
            class={"
                publication
                relative
                self-start
                top-0
                py-4 lg:py-8
                flex flex-col
                gap-0 lg:gap-2
                items-start
                w-full
                text-neutral-500 dark:text-neutral-300
                border-b border-neutral-200 dark:border-neutral-700 "
                (bg_background())
                " "
                (body_text())} {
            p class="translator font-light lg:font-extralight" {
                (translator_names.join(", "))
            }
            p class="publication-name font-light lg:font-thin italic" {
                (publication.name)
            }
            p class="publication-year-and-hide font-light lg:font-thin" {
                span class="publication-year lg:text-lg" {
                    (publication.year_or_unknown())
                }
                " • "
                (Link::new()
                    .class(show_hide.toggle())
                    .class("transition hover:text-neutral-800 dark:hover:text-neutral-100")
                    .slot(html! {
                        span class=(show_hide.show_by_default()) { "hide" }
                        span class=(show_hide.hide_by_default()) { "show" }
                    }))
            }
        }
    )
}

fn body_text() -> &'static str {
    // text-lg lg:text-3xl
    "lg:text-xl"
}

fn poems_in_publication(
    publication: &'static Publication,
    show_hide: &ShowHide,
    initial_poems_loaded: InitiallyLoad,
) -> Markup {
    html! {
        div
            class="poems-in-publication" {
            (visible_poems_in_publication(publication, show_hide, initial_poems_loaded))
            (hidden_poems_in_publication(show_hide))
        }
    }
}

fn visible_poems_in_publication(
    publication: &'static Publication,
    show_hide: &ShowHide,
    initial_poems_loaded: InitiallyLoad,
) -> Markup {
    let show_by_default_class = show_hide.show_by_default();

    let gap = if publication.has_japanese_text() {
        // "gap-8 lg:gap-20"
        "gap-9 lg:gap-12"
    } else {
        // "gap-8 lg:gap-16"
        "gap-9 lg:gap-16"
    };

    let poems: Vec<_> = match initial_poems_loaded {
        InitiallyLoad::AllPoems => publication.poems().collect(),
        InitiallyLoad::PreviewPoems => publication.preview_poems().collect(),
    };

    html! {
        div
            class={"
                visible-poems-in-publication
                flex flex-col
                text-base "
                (gap)
                " "
                (body_text())
                " "
                (show_by_default_class)} {
            @for preview_poem in poems {
                (poem(preview_poem))
            }

            (load_more_poems(publication))
        }
    }
}

fn load_more_poems(publication: &'static Publication) -> Markup {
    if !publication.has_non_preview_poems() {
        return html! {};
    }

    let replace_class = shared::controllers::replace::replace(Route::NonPreviewPoems {
        publication_id: publication.id,
    });

    html!(
        // bg-neutral-200 dark:bg-neutral-600
        div
            class={"
                text-neutral-400 dark:text-neutral-400
                transition hover:text-neutral-600 dark:hover:text-neutral-200
                flex flex-row gap-2 "
                (replace_class)} {

            "⨀"

            button
                class={"
                    tracking-wider
                    whitespace-nowrap
                    font-thin
                    w-min "
                    (link_classes())} {
                "load more"
            }
        }
    )
}

fn hidden_poems_in_publication(show_hide: &ShowHide) -> Markup {
    html! {
        div
            class={"
                hidden-poems
                font-light lg:font-extralight
                text-sm lg:text-base
                text-neutral-400 dark:text-neutral-500
                p-8
                w-full
                border border-neutral-200 dark:border-neutral-700
                rounded
                cursor-pointer "
                (show_hide.hide_by_default())
                " "
                (show_hide.toggle())} {
            "Click to show these poems."
        }
    }
}

pub fn poem(poem: &'static Poem) -> Markup {
    let english_text = poem.english_text.replace("—\n", "—");
    let english_classes =
        "poem-english-text font-light lg:font-extralight text-neutral-600 dark:text-neutral-200";

    match &poem.japanese_text {
        Some(japanese_text) => {
            let romaji_classes = if poem.japanese_text_is_romaji() {
                "italic font-light lg:font-extralight"
            } else {
                "font-light lg:font-thin"
            };

            html!(
                div class="poem flex flex-col gap-1 lg:gap-2 lowercase " {
                    span class=(english_classes) {
                        (english_text)
                    }
                    span
                        class={"
                            poem-japanese-text
                            text-neutral-400 dark:text-neutral-400 "
                            (romaji_classes)} {
                        (japanese_text)
                    }
                }
            )
        }
        None => {
            html!(
                div class="poem flex flex-col lowercase " {
                    span class=(english_classes) {
                        (english_text)
                    }
                }
            )
        }
    }
}

fn hero_image() -> Markup {
    let class = "
        shrink-0 min-w-full min-h-full
        transform dark:-scale-x-100
    ";

    LightDarkImage::new(&ASSET_INDEX.hero_image)
        .class(class)
        .above_the_fold(true)
        .is_largest_contentful_paint(true)
        .render()
}
