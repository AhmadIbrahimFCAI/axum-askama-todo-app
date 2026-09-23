use askama::Template;
use axum::{Router, response::{Html, IntoResponse, Response}, routing::get};


pub fn routes() -> Router{
    Router::new()
        .route("/not-found", get(not_found_handler))
        .route("/server-error", get(server_error_handler))
}


async fn not_found_handler() -> Response{
    let html_string = NotFoundTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


async fn server_error_handler() -> Response{
    let html_string = ServerErrorTemplate{}.render().unwrap();
    Html(html_string).into_response()
}



#[derive(Template)]
#[template(path="pages/errors/not-found.html")]
struct NotFoundTemplate{}


#[derive(Template)]
#[template(path="pages/errors/server-error.html")]
struct ServerErrorTemplate{}