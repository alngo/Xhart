use crate::domain::user::User;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait UserRepository {
    fn create(&self, user: User) -> Result<uuid::Uuid, String>;
    fn get_by_email(&self, email: &str) -> Option<uuid::Uuid>;
    fn get_by_id(&self, id: uuid::Uuid) -> Option<User>;
}
