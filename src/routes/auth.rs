
use askama::Template;
use axum::{Router, response::{Html, IntoResponse, Response}, routing::get};


pub fn routes() -> Router{
    Router::new()
        .route("/log-in", get(login_handler))
        .route("/sign-up", get(signup_handler))
}

async fn login_handler() -> Response{
    let html_string = LogInTemplate{}.render().unwrap();
    Html(html_string).into_response()
}




async fn signup_handler() -> Response{
    let html_string = SignUpTemplate{}.render().unwrap();
    Html(html_string).into_response()
}


#[derive(Template)]
#[template(path="pages/auth/log-in.html")]
struct LogInTemplate{}


#[derive(Template)]
#[template(path="pages/auth/sign-up.html")]
struct SignUpTemplate{}