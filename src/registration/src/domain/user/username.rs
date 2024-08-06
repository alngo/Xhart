use crate::domain::rules::error::BusinessRuleError;
use core::fmt;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, BusinessRuleError> {
        const MIN_LEN: &'static usize = &5;
        const MAX_LEN: &'static usize = &20;
        if username.is_empty() || &username.len() < MIN_LEN || &username.len() > MAX_LEN {
            return Err(BusinessRuleError {
                message: format!("Username ({}) must be valid", username).to_string(),
            });
        }
        Ok(Username(username.to_string()))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod username_tests {
    use super::*;

    #[test]
    fn test_new_invalid_username_empty() {
        let username = "";
        let result = Username::new(username);
        let expected = Err(BusinessRuleError {
            message: "Username () must be valid".to_string(),
        });
        assert!(result.is_err());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_invalid_username_too_short() {
        let username = "a";
        let result = Username::new(username);
        let expected = Err(BusinessRuleError {
            message: "Username (a) must be valid".to_string(),
        });
        assert!(result.is_err());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_invalid_username_too_long() {
        let username = "a".repeat(21);
        let result = Username::new(&username);
        let expected = Err(BusinessRuleError {
            message: "Username (aaaaaaaaaaaaaaaaaaaaa) must be valid".to_string(),
        });
        assert!(result.is_err());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_valid_email() {
        let username = "username";
        let result = Username::new(username);
        assert!(result.is_ok());
    }
}
