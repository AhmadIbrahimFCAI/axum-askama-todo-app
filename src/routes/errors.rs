use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

use crate::data::errors::DataError;



#[derive(Error, Debug)]
pub enum AppError{
    #[error("Database error")]
    Database(#[from] DataError),

    #[error("Template error")]
    Template(#[from] askama::Error)
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(e) => todo!(),
            AppError::Template(e) => todo!(),
        }
    }
}

fn server_error(e: String) -> (StatusCode, Response<Body>){
    tracing::error!("Server error: {}", e);
    
}