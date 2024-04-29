use crate::{models::Email, storage::newsletter::AppState, templates};
use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form,
};
use serde::Deserialize;

pub fn router(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/signup", post(newsletter_signup))
        .route(
            "/confirm",
            post(newsletter_confirm).get(newsletter_confirm_page),
        )
        .route("/confirmed", get(newsletter_confirmed))
        .route("/thanks", get(newsletter_thanks))
        .with_state(app_state)
}

#[derive(Clone, Debug, Deserialize)]
struct SignupRequest {
    email: Email,
}

#[derive(Clone, Debug, Deserialize)]
struct EmailConfirmRequest {
    token: String,
}

#[debug_handler]
async fn newsletter_signup(
    State(app_state): State<AppState>,
    Form(signup_request): Form<SignupRequest>,
) -> Result<Redirect, (StatusCode, String)> {
    app_state
        .newsletter_store
        .signup_for_newsletter(signup_request.email)
        .await
        .map_err(|err| {
            println!("error inserting to the database: {err}");
            (
                StatusCode::BAD_GATEWAY,
                "error signing up, refresh to try again".to_string(),
            )
        })?;
    Ok(Redirect::to("/newsletter/thanks"))
}

async fn newsletter_thanks() -> impl IntoResponse {
    templates::NewsletterThanks
}
#[derive(Default, Debug, Clone)]
pub struct ConfirmedEmail {
    pub email: String,
}

async fn newsletter_confirm(
    State(app_state): State<AppState>,
    Form(confirm_request): Form<EmailConfirmRequest>,
) -> Result<Redirect, (StatusCode, String)> {
    let res = app_state
        .newsletter_store
        .confirm_newsletter_signup(confirm_request.token)
        .await;

    if let Err(e) = res {
        match e {
            sqlx::Error::RowNotFound => Err((StatusCode::BAD_REQUEST, "bad token".to_string())),
            _ => Err((
                StatusCode::BAD_GATEWAY,
                "error saving email address confirmation, refresh to try again".to_string(),
            )),
        }
    } else {
        Ok(Redirect::to("/newsletter/confirmed"))
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct QueryToken {
    pub token: String,
}

async fn newsletter_confirm_page(Query(token): Query<QueryToken>) -> impl IntoResponse {
    templates::NewsletterConfirm { token: token.token }
}

async fn newsletter_confirmed() -> impl IntoResponse {
    templates::NewsletterConfirmed
}
