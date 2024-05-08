# lucaaurelia.com

The source code for [lucaaurelia.com](https://lucaaurelia.com), powered by a custom Rust web framework that prioritizes simplicity, fast page loads, and a minimal carbon footprint.

![landscape](./server/src/assets/images/green_like_no_grass_is_green.png)

### Assets

Assets are prepared using Rust's procedural macros, which means that compiling this project automatically prepares all relevant images, fonts, and other assets. No other build tooling is required besides Cargo.

The macros also generate code for each asset which you can use when loading them from HTML, meaning the Rust compiler will protect you from 404 errors.

The `include_tailwind!` macro will run Tailwind and save the resulting CSS file to the `assets/built` folder.

```rust
let css = assets::include_tailwind!(
    path_to_input_file: "server/src/assets/main.css",
    url_path: "/built-assets/built.css",
    performance_budget_millis: 150,
);

// The asset will be served from the given URL path.
// This will print "/built-assets/built.css".
dbg!(&css.url_path);
```

The `include_image!` macro makes it trivial to follow best practices for image assets, including lossy compression, placeholder generation, and creating resized variants for the use with the HTML `<picture>` tag, which allows the browser to choose the appropriate image file to load at runtime.

```rust
let image = assets::include_image!(
    path_to_image: "server/src/assets/images/image.png",
    alt: "",
    placeholder: automatic_color,
);
```

Other macros are `include_file!` for including arbitrary files with no further processing, useful for things like fonts, and `include_browser_crate!` which will compile the `browser` folder to WASM so you can write client-side Rust instead of client-side JavaScript.

### Santoka

This repo also includes a [dataset of translations](./santoka) of poems by the itinerant Zen poet Taneda Santōka, formatted in JSON for easy manipulation and analysis. This is the dataset that powers [lucaaurelia.com/santoka](https://lucaaurelia.com/santoka).
