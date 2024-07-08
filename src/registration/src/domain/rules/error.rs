use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct BusinessRuleError {
    pub message: String,
}

impl Error for BusinessRuleError {}

impl fmt::Display for BusinessRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplicationError: {}", self.message)
    }
}
