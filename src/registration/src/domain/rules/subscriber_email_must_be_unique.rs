use crate::domain::abstract_repository::Repository;

use super::abstract_rule::BusinessRule;

pub struct SubscriberEmailMustBeUnique<'r, R> {
    subscriber_repository: &'r R,
    email: String,
}

impl<'r, R> SubscriberEmailMustBeUnique<'r, R>
where
    R: Repository,
{
    pub fn new(subscriber_repository: &'r R, email: String) -> Self {
        SubscriberEmailMustBeUnique {
            subscriber_repository,
            email,
        }
    }
}

impl<'r, R> BusinessRule for SubscriberEmailMustBeUnique<'r, R>
where
    R: Repository,
{
    fn is_broken(&self) -> bool {
        self.subscriber_repository
            .get_by_email(&self.email)
            .is_some()
    }

    fn message(&self) -> String {
        "User email must be unique.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockRepository;

    #[test]
    fn test_subscriber_email_must_be_unique() {
        let email = "test@email.com".to_string();

        let mut repository = MockRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);

        assert_eq!(
            false,
            SubscriberEmailMustBeUnique::new(&repository, email).is_broken()
        );
    }

    #[test]
    fn test_subscriber_email_must_be_unique_is_broken() {
        let email = "test@email.com".to_string();

        let mut repository = MockRepository::new();
        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| Some(uuid::Uuid::now_v7()));

        assert_eq!(
            true,
            SubscriberEmailMustBeUnique::new(&repository, email).is_broken()
        );
    }
}
