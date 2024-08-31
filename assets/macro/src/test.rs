use quote::quote;

mod browser_crate;
mod cache;
mod file_change;
mod logger;
mod parse_macro_arguments;

pub fn main() {
    println!("Running");
    let input = quote! {
        path_to_browser_crate_from_workspace_root: "browser",
        js_url_path: "/built-assets/browser.js",
        js_performance_budget_millis: 150,
        wasm_url_path: "/built-assets/browser_bg.wasm",
        wasm_performance_budget_millis: 310,
        production: true,
    };
    let output = browser_crate::include(input);
}