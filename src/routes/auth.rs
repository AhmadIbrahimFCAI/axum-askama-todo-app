
use askama::Template;
use axum::{Form, Router, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, routing::get};
use validator::Validate;
use crate::models::user_form_model::{AuthFormModel};
use super::helpers;

pub fn routes() -> Router{
    Router::new()
        .route("/log-in", get(login_handler))
        .route("/sign-up",
        get(signup_handler)
                      .post(post_sign_up_handler))
}

async fn post_sign_up_handler(Form(user_form): Form<AuthFormModel>) ->Response{
    tracing::info!("Email is {} and the password is {}", user_form.email, user_form.password);
    match user_form.validate(){
        Ok(_) => {
            Redirect::to("/").into_response()
        },
        Err(err) => {
            // println!("{:?}", helpers::extract_errors(&err));
            // tracing::info!("{}", err);
            let err = err.to_string();
            let mut email_error = String::new();
            let mut password_error = String::new();
            helpers::extract_error(&err, |field, message|{
                if field == "email" {
                    email_error = message
                }
                else if field == "password" {
                    password_error = message
                }
            });

            let html_string = SignUpTemplate{
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                
            }.render().unwrap();

            let response = Html(html_string).into_response();
            (StatusCode::BAD_REQUEST, response).into_response()
        }
    }

}



async fn login_handler() -> Response{
    let html_string = LogInTemplate{}.render().unwrap();
    Html(html_string).into_response()
}




async fn signup_handler() -> Response{
    let html_string = SignUpTemplate{
        email: "",
        email_error: "",
        password_error: "",
    }.render().unwrap();
    Html(html_string).into_response()
}


#[derive(Template)]
#[template(path="pages/auth/log-in.html")]
struct LogInTemplate{}


#[derive(Template)]
#[template(path="pages/auth/sign-up.html")]
struct SignUpTemplate<'a>{
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
}