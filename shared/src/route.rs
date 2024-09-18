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

    pub fn is_page(&self) -> bool {
        match self {
            Route::BuildTime => false,
            Route::Email => false,
            Route::Home => true,
            Route::NotFound => true,
            Route::SantokaAllPoems => true,
            Route::SantokaNonPreviewPoems { publication_id: _ } => true,
            Route::Work { id: _ } => true,
        }
    }

    pub fn segments(&self) -> Vec<String> {
        match self {
            Route::BuildTime => vec!["build-time".to_string()],
            Route::Email => vec!["mailto:luca@lucaaurelia.com".to_string()],
            Route::Home => vec![],
            // Route::ImageGarden => "/image-garden".to_string(),
            Route::NotFound => vec!["not-found".to_string()],
            Route::SantokaAllPoems => vec!["santoka".to_string(), "all".to_string()],
            Route::SantokaNonPreviewPoems { publication_id } => {
                vec![
                    "santoka".to_string(),
                    "publications".to_string(),
                    publication_id.to_string(),
                    "non-preview-poems".to_string(),
                ]
            }
            Route::Work { id } => match id {
                WorkId::Santoka => vec![id.url_slug().to_string()],
                _ => vec!["works".to_string(), id.url_slug().to_string()],
            },
        }
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let route_string = "/".to_string() + &self.segments().join("/");
        write!(f, "{}", route_string)
    }
}
