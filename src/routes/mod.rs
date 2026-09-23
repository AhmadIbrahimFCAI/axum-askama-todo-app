use std::time::Duration;

use axum::{Router, body::Body, extract::Request, response::Response};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

pub mod auth;
pub mod global;
pub mod errors;


pub fn create_router() -> Router{
    let v1_routes = Router::new()
        .nest("/auth", auth::routes())
        .nest("/error", errors::routes())
        .merge(global::routes())
        .layer(TraceLayer::new_for_http()
            .make_span_with(|_: &Request<Body>| tracing::info_span!("http-request"))
            .on_request(on_request)
            .on_response(on_response)
            .on_failure(on_failure)
        );
    
    Router::new()
        .merge(v1_routes)
}


fn on_request(request: &Request<Body>, _: &Span){
    tracing::info!("Request started: method {} path {}", request.method(), request.uri().path())
}

fn on_response(response: &Response<Body>, latency: Duration, _: &Span){
    tracing::info!("Response generated: status {} in {:?}", response.status(), latency)
}

fn on_failure(error: ServerErrorsFailureClass, latency: Duration, _: &Span) {
    tracing::error!("Request failed: {:?} after {:?}", error, latency)
}