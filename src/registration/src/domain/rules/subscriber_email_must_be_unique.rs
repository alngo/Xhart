use crate::domain::abstract_repository::Repository;

use super::abstract_rule::BusinessRule;

pub struct UserEmailMustBeUnique<'r, R> {
    subscriber_repository: &'r R,
    email: String,
}

impl<'r, R> UserEmailMustBeUnique<'r, R>
where
    R: Repository,
{
    pub fn new(subscriber_repository: &'r R, email: String) -> Self {
        UserEmailMustBeUnique {
            subscriber_repository,
            email,
        }
    }
}

impl<'r, R> BusinessRule for UserEmailMustBeUnique<'r, R>
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
