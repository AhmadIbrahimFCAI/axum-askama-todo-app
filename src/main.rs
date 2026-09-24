
mod routes;
mod init;
mod models;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind addr");
    let app = routes::create_router();
    
    init::logging();

    init::database_connection().await;

    tracing::info!("Server is starting...");
    tracing::info!("Listening @ {}", addr);

    axum::serve(listener, app).await.expect("Failed to start the server");
}
