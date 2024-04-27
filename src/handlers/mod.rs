use sqlx::{Pool, Postgres};

mod health;
mod home;
mod newsletter;

pub fn router(pool: Pool<Postgres>) -> axum::Router {
    axum::Router::new().nest(
        "/",
        home::router()
            .nest("/health", health::router())
            .nest("/newsletter", newsletter::router(pool)),
    )
}
