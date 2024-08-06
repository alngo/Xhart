use crate::{
    application::abstract_handler::Handler,
    domain::{
        abstract_repository::UserRepository,
        user::{Email, User},
    },
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct SubscribeCommand {
    id: uuid::Uuid,
    username: String,
    email: Email,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct SubscribeResponse {
    id: uuid::Uuid,
}

pub struct Subscribe<'r, R> {
    repository: &'r mut R,
}

impl<'r, R> Subscribe<'r, R>
where
    R: UserRepository,
{
    pub fn new(repository: &'r mut R) -> Self {
        Self { repository }
    }
}

impl<'r, R> Handler for Subscribe<'r, R>
where
    R: UserRepository,
{
    type Request = SubscribeCommand;
    type Response = SubscribeResponse;

    fn handle(&mut self, request: &Self::Request) -> Self::Response {
        let subscriber = User::subscribe(
            request.id,
            &request.username,
            request.email.clone(),
            self.repository,
        );
        self.repository.create(subscriber.unwrap()).unwrap();

        SubscribeResponse { id: request.id }
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
        let email = Email::new("test@email.com").unwrap();
        let mut repository = MockUserRepository::new();

        let request = SubscribeCommand {
            id,
            username,
            email,
        };

        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Ok(id));

        let response = Subscribe::new(&mut repository).handle(&request);

        assert_eq!(response, SubscribeResponse { id });
    }
}
