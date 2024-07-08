#[cfg(test)]
use mockall::automock;

use crate::domain::subscriber::Subscriber;

#[cfg_attr(test, automock)]
pub trait SubscriberRepository {
    fn create(&self, subscriber: Subscriber) -> Result<(), String>;
    fn is_email_unique(&self, email: &String) -> bool;
}

#[cfg_attr(test, automock)]
pub trait IBusinessRule {
    fn is_broken(&self) -> bool;
    fn message(&self) -> String;
}

pub struct UserEmailMustBeUnique<'r, R> {
    subscriber_repository: &'r R,
    email: String,
}

impl<'r, R> UserEmailMustBeUnique<'r, R>
where
    R: SubscriberRepository,
{
    pub fn new(subscriber_repository: &'r R, email: String) -> Self {
        UserEmailMustBeUnique {
            subscriber_repository,
            email,
        }
    }
}

impl<'r, R> IBusinessRule for UserEmailMustBeUnique<'r, R>
where
    R: SubscriberRepository,
{
    fn is_broken(&self) -> bool {
        self.subscriber_repository.is_email_unique(&self.email)
    }

    fn message(&self) -> String {
        "User email must be unique.".to_string()
    }
}
