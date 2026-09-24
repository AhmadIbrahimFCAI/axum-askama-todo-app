use std::time::Duration;

use axum::{Router, body::Body, extract::Request, response::Response};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

pub mod auth;
pub mod global;
pub mod server_errors;
pub mod helpers;


use crate::models::app::AppState;


pub fn create_router(app_state: AppState) -> Router{
    let v1_routes = Router::new()
        .nest("/auth", auth::routes(app_state))
        .nest("/error", server_errors::routes())
        .merge(global::routes())
        .layer(TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| tracing::info_span!(""))
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failure)
        );
    
    Router::new()
        .merge(v1_routes)
}


fn on_request(request: &Request<Body>, _: &Span){
    tracing::info!("-> {} | {}", request.method(), request.uri().path())
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span){
    tracing::info!("<- {} | {:?}", response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("-x {:?} | {:?}", error, latency)
}