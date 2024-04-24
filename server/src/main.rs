use crate::extensions::*;
use axum::{extract::Request, routing::get, Router};
use routes::route::ServerSideRouteExtension;
use shared::route::Route;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

mod assets;
mod components;
mod css_class_groups;
mod extensions;
mod library;
mod routes;

// #[shuttle_runtime::main]
// async fn main() -> shuttle_axum::ShuttleAxum {
//     let built_assets_browser_prefix = {
//         let browser_prefix = paths::built_assets_browser_prefix();
//         format!("/{}", browser_prefix.to_string_lossy())
//     };
//     let built_assets_dir = paths::built_assets_dir();

//     let compression_layer = CompressionLayer::new()
//         .br(true)
//         .deflate(true)
//         .gzip(true)
//         .zstd(true);

//     let app = Router::new()
//         .route("/", get(handle_request)) // The wildcard "/*anthing" syntax doesn't match the root route, so we have to register that one separately.
//         .route("/*anything", get(handle_request))
//         .route("/healthz", get(health_check))
//         .nest_service(
//             &built_assets_browser_prefix,
//             ServeDir::new(built_assets_dir),
//         )
//         .layer(compression_layer);

//     // let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
//     // let host_and_port = format!("0.0.0.0:{}", port);
//     // // Run our app with hyper, listening globally on the specified port.
//     // let listener = tokio::net::TcpListener::bind(&host_and_port).await.unwrap();
//     // axum::serve(listener, app).await.unwrap();

//     // println!("Listening on {}", host_and_port);
//     Ok(app.into())
// }

#[tokio::main]
async fn main() {
    let task = parse_task_from_cli_args();

    match task {
        Task::StartServer => start_server().await,
        Task::SaveObsidianHtmlViews => save_obsidian_html_views(),
        Task::TestLeaflet => library_of_babel::test_leaflet(),
    }
}

async fn start_server() {
    let built_assets_browser_prefix = {
        let browser_prefix = paths::built_assets_browser_prefix();
        format!("/{}", browser_prefix.to_string_lossy())
    };
    let built_assets_dir = paths::built_assets_dir();

    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);

    let app = Router::new()
        .route("/", get(handle_request)) // The wildcard "/*anthing" syntax doesn't match the root route, so we have to register that one separately.
        .route("/*anything", get(handle_request))
        .route("/healthz", get(health_check))
        .nest_service(
            &built_assets_browser_prefix,
            ServeDir::new(built_assets_dir),
        )
        .layer(compression_layer);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host_and_port = format!("0.0.0.0:{}", port);
    // Run our app with hyper, listening globally on the specified port.
    let listener = tokio::net::TcpListener::bind(&host_and_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on {}", host_and_port);
}

// For now, all of our routes return HTML.
async fn handle_request(req: Request) -> axum::response::Html<String> {
    let route = Route::from_request(&req);
    route.html().into_axum_html_response()
}

async fn health_check() {}

fn save_obsidian_html_views() {
    // routes::image_garden::save_to_obsidian_page();
}

fn parse_task_from_cli_args() -> Task {
    let maybe_arg = std::env::args().nth(1);
    match maybe_arg.as_deref() {
        Some("start-server") => Task::StartServer,
        Some("save-obsidian-html-views") => Task::SaveObsidianHtmlViews,
        Some("test-leaflet") => Task::TestLeaflet,
        Some(arg) => {
            panic!("Unkown argument: {arg}");
        }
        None => Task::StartServer,
    }
}

enum Task {
    StartServer,
    SaveObsidianHtmlViews,
    TestLeaflet,
}
