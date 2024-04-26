mod health;
mod home;
mod newsletter;

pub fn router() -> axum::Router {
    axum::Router::new().nest(
        "/",
        home::router()
            .nest("/health", health::router())
            .nest("/newsletter", newsletter::router()),
    )
}
