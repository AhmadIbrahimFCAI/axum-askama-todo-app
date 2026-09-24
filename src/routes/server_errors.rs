use askama::Template;
use axum::{Router, response::{Html, IntoResponse, Response}, routing::get};


pub fn routes() -> Router{
    Router::new()
        .route("/not-found", get(not_found_handler))
        .route("/server-error", get(basic_server_error_handler))
}


async fn not_found_handler() -> Response{
    let html_string = NotFoundTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


async fn basic_server_error_handler() -> Response{
    let html_string = BasicServerErrorTemplate{}.render().unwrap();
    Html(html_string).into_response()
}

// async fn server_error_handler() -> Response{
//     let html_string = ServerErrorTemplate{}.render().unwrap();
//     Html(html_string).into_response()
// }


#[derive(Template)]
#[template(path="pages/errors/not-found.html")]
struct NotFoundTemplate{}


#[derive(Template)]
#[template(path="pages/errors/basic-server-error.html")]
struct BasicServerErrorTemplate{}


#[derive(Template)]
#[template(path="pages/errors/server-error.html")]
pub struct ServerErrorTemplate{}