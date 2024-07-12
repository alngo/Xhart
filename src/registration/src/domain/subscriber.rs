use crate::domain::rules::{error::BusinessRuleError, subscriber_email_must_be_unique::*};

use super::{abstract_entity::Entity, abstract_repository::Repository, rules::subscriber_email_must_be_valid::SubscriberEmailMustBeValid};

pub type Email = String;
pub type Username = String;

pub struct Subscriber {
    username: Username,
    email: Email,
}

impl Entity for Subscriber {}

impl Subscriber {
    pub fn subscribe<'r, R: Repository>(
        username: &Username,
        email: &Email,
        repository: &R,
    ) -> Result<Self, BusinessRuleError> {
        Subscriber::check_rule(SubscriberEmailMustBeUnique::new(repository, email.clone()))?;
        Subscriber::check_rule(SubscriberEmailMustBeValid::new(email.clone()))?;
        Ok(Subscriber {
            username: username.to_string(),
            email: email.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockRepository;

    #[test]
    fn test_subscribe_is_ok() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = MockRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        let result = Subscriber::subscribe(&username, &email, &repository);
        assert!(result.is_ok());
    }

    #[test]
    fn test_subscribe_email_must_be_unique() {
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = MockRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| Some(uuid::Uuid::now_v7()));

        let result = Subscriber::subscribe(&username, &email, &repository);
        assert!(result.is_err());
    }

    #[test]
    fn test_subscribe_email_must_be_valid() {
        let username = "test".to_string();
        let email = "".to_string();

        let mut repository = MockRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        let result = Subscriber::subscribe(&username, &email, &repository);
        assert!(result.is_err());
    }
}
