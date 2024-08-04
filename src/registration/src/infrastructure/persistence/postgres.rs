use crate::domain::{
    abstract_repository::UserRepository,
    user::{User, UserId},
};

type Connection = String;

struct PostgresUserRepository {
    connection: Connection,
}

impl UserRepository for PostgresUserRepository {
    fn create(&mut self, user: User) -> Result<UserId, String> {
        todo!()
    }

    fn get_by_email(&self, email: &str) -> Option<UserId> {
        todo!()
    }
}
