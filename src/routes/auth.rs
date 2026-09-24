
use askama::Template;
use axum::{Form, Router, response::{Html, IntoResponse, Redirect, Response}, routing::get};

use crate::models::user_form_model::{UserFormModel};


pub fn routes() -> Router{
    Router::new()
        .route("/log-in", get(login_handler))
        .route("/sign-up", 
        get(signup_handler)
                      .post(post_sign_up_handler))
}

async fn post_sign_up_handler(Form(user_form): Form<UserFormModel>) ->Response{
    tracing::info!("Email is {} and the password is {}", user_form.email, user_form.password);
    Redirect::to("/").into_response()
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