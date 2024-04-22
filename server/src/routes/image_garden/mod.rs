// use crate::components::{IncludeBodyClasses, Layout};
// use library_of_babel::obsidian::Vault;
// use maud::{html, Markup, Render};
// use once_cell::sync::Lazy;

// // Doing this iteration is slow, so we only compute the HTML once
// // instead of on every page load.
// static HTML: Lazy<Markup> = Lazy::new(|| {
//     let vault = Vault::production_vault();
//     let image_garden = &vault.compile_checked_pages.image_garden_page;
//     let pages_in_image_garden = image_garden.back_links(&vault);
//     let images = pages_in_image_garden
//         .flat_map(|page| page.embedded_images(&vault))
//         .collect::<Vec<_>>();

//     // Layout::new(
//     //     "Image Garden",
//     //     "Image Garden",
//     //     IncludeBodyClasses::No,
//     html! {
//         div {
//             ul class="pt-8 px-8" style="column-width: 320px; column-gap: 2rem;" {
//                 @for image in images {
//                     a class="block w-full pb-8" {
//                         img src=(image.obsidian_src()) class="block w-full";
//                     }
//                 }
//             }
//         }
//     }
//     // ,
//     // )
//     // .render()
// });

// pub fn page() -> Markup {
//     HTML.clone()
// }

// pub fn save_to_obsidian_page() {
//     let html_string = page().0;
//     let mut vault = Vault::production_vault();
//     vault
//         .compile_checked_pages
//         .image_garden_page
//         .set_contents_and_save(html_string);
// }
