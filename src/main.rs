use std::{error::Error, net::SocketAddr, sync::Arc};

use axum::Router;
use canvas_rs::{
    config, handlers,
    storage::{database::Database, newsletter::AppState},
};
use clap::Parser;
use tokio::signal;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variable from .env file
    dotenv::dotenv().ok();

    // load the env variable into config struct
    let config = config::Config::parse();

    let database = Database::new(&config.database_url).await?;

    let app_state = AppState {
        newsletter_store: Arc::new(database.clone()),
    };

    // build our application with a route
    let app = Router::new()
        .nest_service("/", handlers::router(app_state))
        .nest_service("/public", ServeDir::new("public"));

    // bind an address from the env port
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // run it
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// graceful shutdown: this code is taken from
// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
