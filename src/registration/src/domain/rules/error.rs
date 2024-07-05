use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct DomainRuleError {
    pub message: String,
}

impl Error for DomainRuleError {}

impl fmt::Display for DomainRuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApplicationError: {}", self.message)
    }
}
