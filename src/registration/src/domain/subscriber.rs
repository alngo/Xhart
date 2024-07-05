use serde::Deserialize;

use crate::domain::rules::{
    error::DomainRuleError,
    user_email_must_be_unique::*
};

pub type Email = String;
pub type Username = String;

pub trait Entity {
    fn check_rule(rule: impl IBusinessRule) -> Result<(), DomainRuleError> {
        if rule.is_broken() {
            return Err(DomainRuleError { 
                message: rule.message()
            });
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Subscriber {
    username: Username,
    email: Email,
}

impl Entity for Subscriber { }

impl Subscriber {
    pub fn subscribe(username: Username, email: Email, counter: impl IUserCounter) -> Result<Self, DomainRuleError> {
        Subscriber::check_rule(UserEmailMustBeUnique::new(&counter, email.clone()))?;

        Ok(Subscriber {
            username,
            email
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rules::user_email_must_be_unique::MockIUserCounter;

    #[test]
    fn test_subscribe_is_ok() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut counter = MockIUserCounter::new();
        counter.expect_count_user_by_email()
            .times(1)
            .returning(|_| 0);

        let result = Subscriber::subscribe(username, email, counter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_subscribe_email_is_already_taken() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut counter = MockIUserCounter::new();
        counter.expect_count_user_by_email()
            .times(1)
            .returning(|_| 1);

        let result = Subscriber::subscribe(username, email, counter);
        assert!(result.is_err());
    }
}
