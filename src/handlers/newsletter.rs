use axum::{
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form,
};
use serde::Deserialize;

use crate::{models::Email, templates};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/signup", post(newsletter_signup))
        .route("/thanks", get(newsletter_thanks))
}

#[derive(Clone, Debug, Deserialize)]
struct SignupRequest {
    email: Email,
}

async fn newsletter_signup(Form(signup_request): Form<SignupRequest>) -> impl IntoResponse {
    println!("{:?}", signup_request.email);
    Redirect::to("/newsletter/thanks")
}

async fn newsletter_thanks() -> impl IntoResponse {
    templates::NewsletterThanks
}
