use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("invalid email address")]
    InvalidFormat,
}

lazy_static! {
static ref EMAIL_REGEX:Regex  = Regex::new(
            format!("{}{}{}{}{}",
                "^",
                r#"(?P<local>[a-zA-Z0-9.!#$%&'*+/=?^_\x60{|}~-]+)"#,
                "@",
                r#"(?P<domain>[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*)"#,
                "$"
            ).as_str()).unwrap();
}

#[derive(Clone, Debug)]
pub struct Email(String);

impl Email {
    pub fn is_valid(value: String) -> bool {
        match EMAIL_REGEX.is_match(&value) {
            Ok(true) => true,
            _ => false,
        }
    }
}
impl TryFrom<String> for Email {
    type Error = EmailError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        // email_regex for valid email addresses.
        // See https://regex101.com/r/1BEPJo/latest for an interactive breakdown of the regexp.
        // See https://html.spec.whatwg.org/#valid-e-mail-address for the definition.

        match Email::is_valid(value.clone()) {
            true => return Ok(Email(value)),
            _ => Err(EmailError::InvalidFormat),
        }
    }
}

impl Into<String> for Email {
    fn into(self) -> String {
        return self.0;
    }
}

impl<'de> Deserialize<'de> for Email {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email_str = String::deserialize(deserializer)?;
        Email::try_from(email_str).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_emails() {
        let tests = vec![
            ("me@example.com".to_string(), true),
            ("me@example".to_string(), true),
            ("@example.com".to_string(), false),
            ("me@".to_string(), false),
            ("@".to_string(), false),
            ("".to_string(), false),
            ("me".to_string(), false),
            ("me@example..example".to_string(), false),
            ("me@example.".to_string(), false),
        ];

        for test in tests {
            assert_eq!(Email::is_valid(test.0), test.1)
        }
    }
}
