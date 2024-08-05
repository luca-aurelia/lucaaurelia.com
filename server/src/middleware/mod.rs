use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

pub async fn redirect_trailing_slash(request: Request, next: Next) -> Response {
    let path = request.uri().path();
    if path.ends_with('/') && path != "/" {
        let mut new_path = path.to_string();
        new_path.pop(); // Remove the trailing slash

        let response = Redirect::permanent(&new_path).into_response();
        return response;
    }

    next.run(request).await
}
