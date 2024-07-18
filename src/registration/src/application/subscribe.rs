use crate::domain::abstract_repository::UserRepository;
use crate::domain::user::User;

use serde::{Deserialize, Serialize};

use super::abstract_handler::Handler;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct SubscribeCommand {
    id: uuid::Uuid,
    username: String,
    email: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct SubscribeResponse {
    id: uuid::Uuid,
}

pub struct Subscribe<'r, R> {
    repository: &'r R,
}

impl<'r, R> Subscribe<'r, R>
where
    R: UserRepository,
{
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R> Handler for Subscribe<'r, R>
where
    R: UserRepository,
{
    type Command = SubscribeCommand;
    type Response = SubscribeResponse;

    fn handle(&self, command: &Self::Command) -> Self::Response {
        let subscriber = User::subscribe(command.id, &command.username, &command.email, self.repository);
        self.repository.create(subscriber.unwrap()).unwrap();
        
        SubscribeResponse {
            id: command.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockUserRepository;

    #[test]
    fn test_subscribe() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = "test@email.com".to_string();
        let mut repository = MockUserRepository::new();

        let request = SubscribeCommand { id, username, email };

        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Ok(id));

        let response = Subscribe::new(&repository).handle(&request);

        assert_eq!(
            response,
            SubscribeResponse {
                id,
            }
        );
    }
}
