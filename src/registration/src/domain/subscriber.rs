use serde::Deserialize;

use crate::domain::rules::{error::DomainRuleError, subscriber_email_must_be_unique::*};

pub type Email = String;
pub type Username = String;

pub trait Entity {
    fn check_rule(rule: impl IBusinessRule) -> Result<(), DomainRuleError> {
        if rule.is_broken() {
            return Err(DomainRuleError {
                message: rule.message(),
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

impl Entity for Subscriber {}

impl Subscriber {
    pub fn subscribe(
        username: Username,
        email: Email,
        repository: impl SubscriberRepository,
    ) -> Result<Self, DomainRuleError> {
        Subscriber::check_rule(UserEmailMustBeUnique::new(&repository, email.clone()))?;

        Ok(Subscriber { username, email })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rules::subscriber_email_must_be_unique::SubscriberRepository;

    #[test]
    fn test_subscribe_is_ok() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = SubscriberRepository::new();
        repository
            .expect_is_email_unique()
            .times(1)
            .returning(|_| true);

        let result = Subscriber::subscribe(username, email, repository);
        assert!(result.is_ok());
    }

    #[test]
    fn test_subscribe_email_must_be_unique() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = SubscriberRepository::new();
        repository
            .expect_is_email_unique()
            .times(1)
            .returning(|_| false);

        let result = Subscriber::subscribe(username, email, repository);
        assert!(result.is_err());
    }
}
