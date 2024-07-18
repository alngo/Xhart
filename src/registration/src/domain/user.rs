use super::{
    abstract_entity::Entity,
    abstract_repository::UserRepository,
    rules::{
        error::BusinessRuleError, user_email_must_be_unique::UserEmailMustBeUnique,
        user_email_must_be_valid::UserEmailMustBeValid,
    },
};

pub type Email = String;
pub type Username = String;

pub struct User {
    id: uuid::Uuid,
    username: Username,
    email: Email,
}

impl Entity for User {}

impl User {
    pub fn subscribe<'r, R: UserRepository>(
        id: uuid::Uuid,
        username: &Username,
        email: &Email,
        repository: &R,
    ) -> Result<Self, BusinessRuleError> {
        User::check_rule(UserEmailMustBeValid::new(&email))?;
        User::check_rule(UserEmailMustBeUnique::new(repository, &email))?;
        Ok(User {
            id,
            username: username.to_string(),
            email: email.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockUserRepository;

    #[test]
    fn test_subscribe_is_ok() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        let result = User::subscribe(id, &username, &email, &repository);
        assert!(result.is_ok());
    }

    #[test]
    fn test_subscribe_email_must_be_unique() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = "test@email.com".to_string();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| Some(uuid::Uuid::now_v7()));

        let result = User::subscribe(id, &username, &email, &repository);
        assert!(result.is_err());
    }

    #[test]
    fn test_subscribe_email_must_be_valid() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = "".to_string();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(0)
            .returning(|_| None);

        let result = User::subscribe(id, &username, &email, &repository);
        assert!(result.is_err());
    }
}
