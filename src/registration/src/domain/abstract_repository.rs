#[cfg(test)]
use mockall::automock;

use super::user::{Email, User, UserId};

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn create(&mut self, user: User) -> Result<UserId, String>;
    fn get_by_email(&self, email: &Email) -> Option<UserId>;
}
