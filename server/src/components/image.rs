use std::fmt::Display;

use assets::{ImageAsset, Placeholder};
use maud::{html, Markup, Render};

pub struct Image<'a> {
    pub asset: &'a ImageAsset,
    pub class: &'a str,
    pub loading: Loading,
}

pub enum Loading {
    Lazy,
    Eager,
}

impl Display for Loading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Loading::Lazy => write!(f, "lazy"),
            Loading::Eager => write!(f, "eager"),
        }
    }
}

impl<'a> Image<'a> {
    pub fn new(asset: &'a ImageAsset) -> Self {
        Self {
            asset,
            class: "",
            loading: Loading::Lazy,
        }
    }

    #[allow(dead_code)]
    pub fn class(mut self, class: impl Into<&'a str>) -> Self {
        self.class = class.into();
        self
    }

    pub fn loading(mut self, loading: Loading) -> Self {
        self.loading = loading;
        self
    }
}

impl Render for Image<'_> {
    fn render(&self) -> Markup {
        match &self.asset.placeholder {
            Placeholder::Color { css_string } => {
                image_with_color_placeholder(self.class, self.asset, &self.loading, css_string)
            }
            Placeholder::Lqip { data_uri } => {
                image_with_lqip(self.class, self.asset, &self.loading, data_uri)
            }
        }
    }
}

fn image_with_color_placeholder(
    class: &str,
    asset: &ImageAsset,
    loading: &Loading,
    placeholder_color_css_string: &str,
) -> Markup {
    html!(img
        class={(class) " select-none"}
        style={"background-color: " (placeholder_color_css_string)}
        alt=(asset.alt)
        src=(asset.src)
        srcset=(asset.srcset)
        loading=(loading);
    )
}

fn image_with_lqip(class: &str, asset: &ImageAsset, loading: &Loading, data_uri: &str) -> Markup {
    html!(
        div
            class={"select-none relative overflow-hidden " (class)}
        {

            // LQIP.
            img
                alt=(asset.alt)
                class="shrink-0 min-w-full min-h-full object-cover"
                style="image-rendering: pixelated; image-rendering: -moz-crisp-edges; image-rendering: crisp-edges;"
                src=(data_uri);

            // Actual image.
            img
                alt=(asset.alt)
                class="absolute top-0 left-0 min-w-full min-h-full object-cover"
                src=(asset.src)
                srcset=(asset.srcset)
                loading=(loading);
        }
    )
}
