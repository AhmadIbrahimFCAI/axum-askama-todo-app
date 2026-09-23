use askama::Template;
use axum::{Router, response::{Html, IntoResponse, Response}, routing::get};
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    let static_dir = ServeDir::new("static");

    Router::new()
        .route("/", get(home_handler))
        .route("/create", get(create_handler))
        .route("/todos", get(todos_handler))
        .nest_service("/static", static_dir)
        
}


async fn home_handler() -> Response{
    let html_string = HomeTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


async fn create_handler() -> Response{
    let html_string = CreateTemplate{}.render().unwrap();
    Html(html_string).into_response()
}

async fn todos_handler() -> Response{
    let html_string = TodosTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


#[derive(Template)]
#[template(path="pages/home.html")]
struct HomeTemplate{}

#[derive(Template)]
#[template(path="pages/create.html")]
struct CreateTemplate{}

#[derive(Template)]
#[template(path="pages/todos.html")]
struct TodosTemplate{}
