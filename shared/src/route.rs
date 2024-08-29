use crate::work_id::WorkId;
use enum_iterator;
use santoka::PublicationId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, enum_iterator::Sequence, Serialize, Deserialize, Debug)]
pub enum Route {
    BuildTime,
    Email,
    Home,
    NotFound,
    SantokaAllPoems,
    SantokaNonPreviewPoems { publication_id: PublicationId },
    Work { id: WorkId },
}

impl Route {
    pub fn all() -> impl Iterator<Item = Route> {
        enum_iterator::all::<Route>()
    }

    pub fn parse_path(path: &str) -> Route {
        Route::all()
            .find(|route| route.to_string() == path)
            .unwrap_or(Route::NotFound)
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route_str = match self {
            Route::BuildTime => "/build-time".to_string(),
            Route::Email => "mailto:luca@lucaaurelia.com".to_string(),
            Route::Home => "/".to_string(),
            // Route::ImageGarden => "/image-garden".to_string(),
            Route::NotFound => "/not-found".to_string(),
            Route::SantokaAllPoems => "/santoka/all".to_string(),
            Route::SantokaNonPreviewPoems { publication_id } => {
                format!("/santoka/publications/{}/non-preview-poems", publication_id)
            }
            Route::Work { id } => match id {
                WorkId::Santoka => format!("/{}", id.url_slug()),
                _ => format!("/works/{}", id.url_slug()),
            },
        };

        write!(f, "{}", route_str)
    }
}
