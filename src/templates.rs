use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "newsletter_thanks.html")]
pub struct NewsletterThanks;

#[derive(Template)]
#[template(path = "newsletter_confirmed.html")]
pub struct NewsletterConfirmed;

#[derive(Template)]
#[template(path = "newsletter_confirm.html")]
pub struct NewsletterConfirm {
    pub token: String,
}
