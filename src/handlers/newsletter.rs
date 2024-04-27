use crate::{models::Email, templates};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form,
};
use hex::ToHex;
use rand::RngCore;
use serde::Deserialize;
use sqlx::{PgPool, Pool, Postgres};

pub fn router(pool: Pool<Postgres>) -> axum::Router {
    axum::Router::new()
        .route("/signup", post(newsletter_signup))
        .route(
            "/confirm",
            post(newsletter_confirm).get(newsletter_confirm_page),
        )
        .route("/confirmed", get(newsletter_confirmed))
        .route("/thanks", get(newsletter_thanks))
        .with_state(pool)
}

#[derive(Clone, Debug, Deserialize)]
struct SignupRequest {
    email: Email,
}

#[derive(Clone, Debug, Deserialize)]
struct EmailConfirmRequest {
    token: String,
}

async fn newsletter_signup(
    State(pool): State<PgPool>,
    Form(signup_request): Form<SignupRequest>,
) -> Result<Redirect, (StatusCode, String)> {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    sqlx::query!(
        r#"
            insert into newsletter_subscribers (email, token)
            values ($1, $2)
            on conflict (email) do update set
                token = excluded.token,
                updated = now()
        "#,
        signup_request.email.to_string(),
        secret.encode_hex::<String>()
    )
    .execute(&pool)
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
    State(pool): State<PgPool>,
    Form(confirm_request): Form<EmailConfirmRequest>,
) -> Result<Redirect, (StatusCode, String)> {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    let res = sqlx::query_as!(
        ConfirmedEmail,
        r#"
		update newsletter_subscribers
		set confirmed = true
		where token = $1
		returning email
        "#,
        confirm_request.token,
    )
    .fetch_one(&pool)
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
