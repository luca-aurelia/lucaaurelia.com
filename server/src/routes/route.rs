use super::*;
use axum::extract::Request;
use maud::Markup;
use std::path::Path;

pub use shared::route::Route;

pub trait ServerSideRouteExtension {
    fn from_request(req: &Request) -> Self;
    fn html(&self) -> Markup;
    /// For static site generation.
    fn save(&self, path: &Path);
}

impl ServerSideRouteExtension for Route {
    fn from_request(req: &Request) -> Route {
        let uri_path = req.uri().path();
        Route::parse_path(uri_path)
    }

    fn html(&self) -> Markup {
        match self {
            Route::BuildTime => build_time::page(), // Should be a 404.
            Route::Email => not_found::page(),
            Route::Home => crate::routes::page(),
            // Route::ImageGarden => image_garden::page(),
            Route::NotFound => not_found::page(),
            Route::NonPreviewPoems { publication_id } => {
                santoka::non_preview_poems::page(*publication_id)
            }
            Route::Santoka => santoka::page(),
            Route::AllSantokaPoems => santoka::all::page(),
            Route::Work => work::page(),
        }
    }

    fn save(&self, path: &Path) {
        let html_string = self.html().into_string();
        std::fs::write(path, html_string).unwrap();
    }
}

// This code is WIP.
// use http::method::Method;
//
// trait Route<Input: RouteInput, Response> {
//     fn response(&self, input: Input) -> Response;

//     fn matches_request(&self, request: Request) -> bool {
//         self.uri() == request.uri() && self.method() == request.method()
//     }
//     fn method(&self) -> Method;
//     fn uri(&self) -> String;
// }

// trait RouteInput {
//     fn from_request_params(params: RequestParams) -> Self;
//     fn into_request_params(&self) -> RequestParams;
// }

// struct RequestParams {
//     url: String,
//     method_with_body: MethodWithBody,
// }

// pub enum MethodWithBody {
//     Get,
//     Post { body: String },
//     Put { body: String },
//     Delete,
// }

// struct HomePageRoute {}

// impl Route<(), Markup> for HomePageRoute {
//     fn method(&self) -> Method {
//         Method::GET
//     }

//     fn uri(&self) -> String {
//         "/".to_string()
//     }

//     fn get_response(&self, _: ()) -> Markup {
//         home_page()
//     }
// }
