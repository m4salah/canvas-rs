use hex::ToHex;
use rand::RngCore;
use sqlx::PgPool;

use crate::models::Email;
use async_trait::async_trait;

use super::newsletter::{ConfirmSignup, NewsletterSignup, Signup};

#[derive(Clone)]
pub struct Database {
    pub db: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        // initialize the database pool
        let pool = PgPool::connect(database_url).await?;
        return Ok(Self { db: pool });
    }
}

impl NewsletterSignup for Database {}

#[async_trait]
impl Signup for Database {
    async fn signup_for_newsletter(&self, email: Email) -> Result<String, anyhow::Error> {
        let secret = create_secret();
        sqlx::query!(
            r#"
            insert into newsletter_subscribers (email, token)
            values ($1, $2)
            on conflict (email) do update set
                token = excluded.token,
                updated = now()
        "#,
            email.to_string(),
            secret.clone()
        )
        .execute(&self.db)
        .await?;
        Ok(secret)
    }
}

#[async_trait]
impl ConfirmSignup for Database {
    async fn confirm_newsletter_signup(&self, token: String) -> Result<Email, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            update newsletter_subscribers
            set confirmed = true
            where token = $1
            returning email
        "#,
            token,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Email::try_from(res.email).unwrap())
    }
}

fn create_secret() -> String {
    let mut secret = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);
    secret.encode_hex::<String>()
}
