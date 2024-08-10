use crate::components::{HeroContent, HeroSection, Image, Layout};
use maud::{html, Markup, Render};
use once_cell::sync::Lazy;
use assets::ImageAsset;

struct AssetIndex {
    image_one: ImageAsset,
    image_two: ImageAsset,
    image_three: ImageAsset,
}

static ASSET_INDEX: Lazy<AssetIndex> = Lazy::new(|| {
    AssetIndex {
        image_one: assets::include_image!(
            path_to_image: "server/src/assets/images/fuji river - kawase hasui - 1933.jpeg",
            alt: "",
            placeholder: automatic_color,
        ),
        image_two: assets::include_image!(
            path_to_image: "server/src/assets/images/kominato, boshu - kawase hasui.jpeg",
            alt: "",
            placeholder: automatic_color,
        ),
        image_three: assets::include_image!(
            path_to_image: "server/src/assets/images/izu dogashima - kawase hasui.jpeg",
            alt: "",
            placeholder: automatic_color,
        ),
    }
});

pub fn page() -> Markup {
    Layout::new(
        "Luca Aurelia | Not found",
        "Not found.",
        html! {
            main class="w-full" {
                (HeroSection::new()
                    .class("w-full")
                    .slot(content()))
            }
        },
    )
    .open_graph_image(&ASSET_INDEX.image_two)
    .render()
}

fn content() -> Markup {
    HeroContent::new()
        .class("relative")
        .slot(html! {
            (Image::new(&ASSET_INDEX.image_two)
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
