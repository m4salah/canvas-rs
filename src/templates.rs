use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "newsletter_thanks.html")]
pub struct NewsletterThanks;
