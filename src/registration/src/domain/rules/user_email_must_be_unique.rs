use crate::domain::{abstract_repository::UserRepository, user::Email};

use super::abstract_rule::BusinessRule;

pub struct UserEmailMustBeUnique<'r, 'e, R> {
    user_repository: &'r R,
    email: &'e Email,
}

impl<'r, 'e, R> UserEmailMustBeUnique<'r, 'e, R>
where
    R: UserRepository,
{
    pub fn new(user_repository: &'r R, email: &'e Email) -> Self {
        UserEmailMustBeUnique {
            user_repository,
            email,
        }
    }
}

impl<'r, 's, R> BusinessRule for UserEmailMustBeUnique<'r, 's, R>
where
    R: UserRepository,
{
    fn is_broken(&self) -> bool {
        self.user_repository.get_by_email(&self.email).is_some()
    }

    fn message(&self) -> String {
        "User email must be unique.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockUserRepository;

    #[test]
    fn test_subscriber_email_must_be_unique() {
        let email = Email::new("test@email.com").unwrap();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        assert_eq!(
            false,
            UserEmailMustBeUnique::new(&repository, &email).is_broken()
        );
    }

    #[test]
    fn test_subscriber_email_must_be_unique_is_broken() {
        let email = Email::new("test@email.com").unwrap();

        let mut repository = MockUserRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| Some(uuid::Uuid::now_v7()));

        assert_eq!(
            true,
            UserEmailMustBeUnique::new(&repository, &email).is_broken()
        );
    }
}
