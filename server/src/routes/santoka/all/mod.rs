use super::InitiallyLoad;
use maud::Markup;

pub fn page() -> Markup {
    crate::routes::santoka::page_with_options(InitiallyLoad::AllPoems)
}
