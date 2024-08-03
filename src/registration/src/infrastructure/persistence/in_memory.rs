use crate::domain::{
    abstract_repository::UserRepository,
    user::{User, UserId},
};

struct InMemoryUserRepository {
    users: Vec<User>,
}

impl UserRepository for InMemoryUserRepository {
    fn create(&self, user: User) -> Result<UserId, String> {
        todo!()
    }

    fn get_by_email(&self, email: &str) -> Option<UserId> {
        todo!()
    }
}
