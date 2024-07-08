use crate::domain::subscriber::Subscriber;

#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

#[cfg_attr(test, automock)]
pub trait Repository {
    fn create(&self, subscriber: Subscriber) -> Result<(), String>;
    fn is_email_unique(&self, email: String) -> Result<bool, String>;
}

pub trait UseCase {
    type Request;
    type Response;

    fn execute(&self, request: Self::Request) -> Self::Response;
}

#[derive(Deserialize)]
struct SubscribeRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct SubscribeResponse {
    name: String,
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

    fn execute(&self, request: Self::Request) -> Self::Response {
        let subscriber = Subscriber::subscribe(request.name, request.email, self.repository);
        self.repository.create(subscriber).unwrap();

        SubscribeResponse {
            name: request.name,
            email: request.email,
        }
    }
}
