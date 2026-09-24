
use askama::Template;
use axum::{Form, Router, extract::State, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, routing::get};
use tower_sessions::Session;
use validator::Validate;
use crate::{data::errors::DataError, models::{app::AppState, user_form_model::AuthFormModel}, routes::errors::AppError};
use super::helpers;
use crate::data::user;


pub fn routes(app_state: AppState) -> Router{
    Router::new()
        .route("/log-in", get(login_handler).post(post_login_handler))
        .route("/sign-up",
        get(signup_handler)
                      .post(post_sign_up_handler))
        .with_state(app_state)
}

async fn post_sign_up_handler(
    State(app_state): State<AppState>,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError>{

    tracing::info!("Email is {} and the password is {}", user_form.email, user_form.password);
    match user_form.validate(){
        Ok(_) => {

            let result = user::create_user(
                &app_state.connection_pool, 
                &user_form.email, 
                &user_form.password,
            ).await;

            if let Err(err) = result{
                if let DataError::FailedQuery(e) = err {
                    tracing::error!("Failed to sign up {}", e);
                    return Ok(Redirect::to("/auth/sign-up").into_response());
                } else {
                    Err(err)?
                }
            }

            Ok(Redirect::to("/auth/log-in").into_response())
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
                
            }.render()?;

            let response = Html(html_string).into_response();
            Ok((StatusCode::BAD_REQUEST, response).into_response())
        }
    }

}



async fn login_handler() -> Result<Response, AppError>{
    let html_string = LogInTemplate{
        email: "",
        email_error: "",
        password_error: "",
    }.render()?;
    
    Ok(Html(html_string).into_response())
}



async fn post_login_handler(
    State(app_state): State<AppState>,
    session: Session,
    Form(user_form): Form<AuthFormModel>,
) -> Result<Response, AppError>{

    match user_form.validate(){
        Ok(_) => {
            let user_id = user::authenticate_user(
                &app_state.connection_pool, 
                &user_form.email,
                &user_form.password
            ).await;

            match user_id {
                Ok(user_id) => {
                    session.insert("authenticated_user_id", user_id).await?;
                    Ok(Redirect::to("/todos").into_response())
                },
                Err(_) => {
                    Ok(Redirect::to("/todos").into_response())
                },
            }
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

            let html_string = LogInTemplate{
                email: &user_form.email,
                email_error: &email_error,
                password_error: &password_error,
                
            }.render()?;

            let response = Html(html_string).into_response();
            Ok((StatusCode::BAD_REQUEST, response).into_response())

        }
    }

}





async fn signup_handler() -> Result<Response, AppError>{
    let html_string = SignUpTemplate{
        email: "",
        email_error: "",
        password_error: "",
    }.render()?;
    Ok(Html(html_string).into_response())
}


#[derive(Template)]
#[template(path="pages/auth/log-in.html")]
struct LogInTemplate<'a>{
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
}

#[derive(Template)]
#[template(path="pages/auth/sign-up.html")]
struct SignUpTemplate<'a>{
    pub email: &'a str,
    pub email_error: &'a str,
    pub password_error: &'a str,
}