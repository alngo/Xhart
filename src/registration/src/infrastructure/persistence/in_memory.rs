use crate::domain::{
    abstract_repository::UserRepository,
    user::{User, UserId},
};

struct InMemoryUserRepository {
    users: Vec<User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn create(&mut self, user: User) -> Result<UserId, String> {
        let id = user.id;
        self.users.push(user);
        Ok(id)
    }

    fn get_by_email(&self, email: &str) -> Option<UserId> {
        self.users
            .iter()
            .find(|user| user.email.eq(email))
            .map(|user| user.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_should_add_a_user() {
        let mut repository = InMemoryUserRepository::new();
        let uuid = uuid::Uuid::now_v7();
        let user = User::subscribe(
            uuid,
            &"username".to_string(),
            &"email@email.com".to_string(),
            &mut repository,
        );
        let result = repository.create(user.unwrap());
        assert_eq!(repository.users.len(), 1);
        assert_eq!(result.unwrap(), uuid);
    }

    #[test]
    fn test_get_by_email_existing() {
        let mut repository = InMemoryUserRepository::new();
        let uuid = uuid::Uuid::now_v7();
        let user = User::subscribe(
            uuid,
            &"username".to_string(),
            &"email@email.com".to_string(),
            &mut repository,
        );
        let result = repository.create(user.unwrap());
        let id = repository.get_by_email("email@email.com");
        assert_eq!(result.unwrap(), id.unwrap());
    }

    #[test]
    fn test_get_by_email_non_existing() {
        let epository = InMemoryUserRepository::new();
        let result = repository.get_by_email("email@email.com");
        assert!(result.is_none());
    }
}
