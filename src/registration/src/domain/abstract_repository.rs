#[cfg(test)]
use mockall::automock;

use super::user::{User, UserId};

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn create(&self, user: User) -> Result<UserId, String>;
    fn get_by_email(&self, email: &str) -> Option<UserId>;
}
