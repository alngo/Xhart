use crate::domain::rules::error::BusinessRuleError;
use core::fmt;
use regex::Regex;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Email(String);

impl Email {
    const EMAIL_REGEX: &'static str =
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

    pub fn new(email: &str) -> Result<Self, BusinessRuleError> {
        if email.is_empty() || !Regex::new(Self::EMAIL_REGEX).unwrap().is_match(&email) {
            return Err(BusinessRuleError {
                message: format!("User email ({}) must be valid", email).to_string(),
            });
        }
        Ok(Email(email.to_string()))
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod email_tests {
    use super::*;

    #[test]
    fn test_new_invalid_email() {
        let email = "invalid-email";
        let result = Email::new(email);
        let expected = Err(BusinessRuleError {
            message: "User email (invalid-email) must be valid".to_string(),
        });
        assert!(result.is_err());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_valid_email() {
        let email = "a@valid.email";
        let result = Email::new(email);
        assert!(result.is_ok());
    }
}
