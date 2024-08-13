// We don't actually need this -- the built version of main.css
// gets included when we use include_tailwind!(). But by putting
// it here, Cargo knows we need to rebuild this crate when we change
// main.css.
static _MAIN_CSS: &str = include_str!("./main.css");

pub mod processed_2023_haiku;
