mod email;
mod username;
pub use self::email::Email;

use super::{
    abstract_entity::Entity,
    abstract_repository::UserRepository,
    rules::{error::BusinessRuleError, user_email_must_be_unique::UserEmailMustBeUnique},
};

pub type UserId = uuid::Uuid;
pub type Username = String;

pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
}

impl Entity for User {}

impl User {
    pub fn subscribe<'r, R: UserRepository>(
        id: uuid::Uuid,
        username: &Username,
        email: Email,
        repository: &R,
    ) -> Result<Self, BusinessRuleError> {
        User::check_rule(UserEmailMustBeUnique::new(repository, &email))?;
        Ok(User {
            id,
            username: username.to_string(),
            email,
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
        let email = Email::new("test@email.com").unwrap();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        let result = User::subscribe(id, &username, email, &repository);
        assert!(result.is_ok());
    }

    #[test]
    fn test_subscribe_email_must_be_unique() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = Email::new("test@email.com").unwrap();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| Some(uuid::Uuid::now_v7()));

        let result = User::subscribe(id, &username, email, &repository);
        assert!(result.is_err());
    }
}
