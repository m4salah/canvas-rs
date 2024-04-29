use std::sync::Arc;

use async_trait::async_trait;

use crate::models::Email;

// TODO: How to pass this into the handlers?
pub trait NewsletterStore: NewsletterSignup {}

///
/// Trait for signup and confirm signup for the user
///
/// This trait is implemented by the different storage backends. It provides
/// the basic interface for signup user to the newsletter.
///
pub trait NewsletterSignup: Signup + ConfirmSignup {}
#[async_trait]
pub trait Signup: Send + Sync {
    async fn signup_for_newsletter(&self, email: Email) -> Result<String, anyhow::Error>;
}

#[async_trait]
pub trait ConfirmSignup: Send + Sync {
    async fn confirm_newsletter_signup(&self, token: String) -> Result<Email, sqlx::Error>;
}

#[derive(Clone)]
pub struct AppState {
    pub newsletter_store: Arc<dyn NewsletterStore>,
}
