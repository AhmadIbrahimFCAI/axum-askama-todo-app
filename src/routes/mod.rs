use axum::{Router};

pub mod auth;
pub mod global;
pub mod errors;


pub fn create_router() -> Router{
    let v1_routes = Router::new()
        .nest("/auth", auth::routes())
        .nest("/error", errors::routes())
        .merge(global::routes())
        ;
    
    Router::new()
        .merge(v1_routes)
}