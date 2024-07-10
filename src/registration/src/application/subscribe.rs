use crate::domain::abstract_repository::Repository;
use crate::domain::subscriber::Subscriber;

use serde::{Deserialize, Serialize};

use super::abstract_use_case::UseCase;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct SubscribeRequest {
    username: String,
    email: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct SubscribeResponse {
    id: uuid::Uuid,
    username: String,
    email: String,
}

pub struct Subscribe<'r, R> {
    repository: &'r R,
}

impl<'r, R> Subscribe<'r, R>
where
    R: Repository,
{
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R> UseCase for Subscribe<'r, R>
where
    R: Repository,
{
    type Request = SubscribeRequest;
    type Response = SubscribeResponse;

    fn execute(&self, request: &Self::Request) -> Self::Response {
        let subscriber = Subscriber::subscribe(&request.username, &request.email, self.repository);
        let id = self.repository.create(subscriber.unwrap()).unwrap();

        SubscribeResponse {
            id,
            username: request.username.clone(),
            email: request.email.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::abstract_repository::MockRepository;

    #[test]
    fn test_subscribe() {
        let id = uuid::Uuid::now_v7();
        let username = "test".to_string();
        let email = "test@email.com".to_string();
        let mut repository = MockRepository::new();

        let request = SubscribeRequest { username, email };

        repository
            .expect_get_by_email()
            .times(1)
            .returning(|_| None);
        repository
            .expect_create()
            .times(1)
            .returning(move |_| Ok(id));

        let response = Subscribe::new(&repository).execute(&request);

        assert_eq!(
            response,
            SubscribeResponse {
                id,
                username: request.username,
                email: request.email,
            }
        );
    }
}
