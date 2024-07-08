use crate::domain::subscriber::Subscriber;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Repository {
    fn create(&self, subscriber: Subscriber) -> Result<uuid::Uuid, String>;
    fn get_by_email(&self, email: &str) -> Option<uuid::Uuid>;
}
