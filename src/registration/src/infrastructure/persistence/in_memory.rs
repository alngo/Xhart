use crate::domain::{
    abstract_repository::UserRepository,
    user::{Email, User, UserId},
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

    fn get_by_email(&self, email: &Email) -> Option<UserId> {
        self.users
            .iter()
            .find(|user| user.email.eq(email))
            .map(|user| user.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::user::Email;

    use super::*;

    #[test]
    fn test_create_should_add_a_user() {
        let mut repository = InMemoryUserRepository::new();
        let uuid = uuid::Uuid::now_v7();
        let user = User::subscribe(
            uuid,
            &"username".to_string(),
            Email::new("email@email.com").unwrap(),
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
        let email = Email::new("email@email.com").unwrap();
        let user = User::subscribe(
            uuid,
            &"username".to_string(),
            email.clone(),
            &mut repository,
        );
        let result = repository.create(user.unwrap());
        let id = repository.get_by_email(&email);
        assert_eq!(result.unwrap(), id.unwrap());
    }

    #[test]
    fn test_get_by_email_non_existing() {
        let repository = InMemoryUserRepository::new();
        let email = Email::new("email@email.com").unwrap();
        let result = repository.get_by_email(&email);
        assert!(result.is_none());
    }
}
