use crate::storage::newsletter::AppState;

mod health;
mod home;
mod newsletter;

pub fn router(app_state: AppState) -> axum::Router {
    axum::Router::new().nest(
        "/",
        home::router()
            .nest("/health", health::router())
            .nest("/newsletter", newsletter::router(app_state)),
    )
}
