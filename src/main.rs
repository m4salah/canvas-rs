use axum::Router;
use tower_http::services::ServeDir;
mod handlers;
mod models;
mod templates;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .nest_service("/", handlers::router())
        .nest_service("/public", ServeDir::new("public"));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
