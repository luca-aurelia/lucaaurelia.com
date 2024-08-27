use maud::{html, Markup};
use shared::work::Work;

pub fn page(work: &Work) -> Markup {
    html! {
        h1 {
            (work.name.human_readable())
        }
    }
}
