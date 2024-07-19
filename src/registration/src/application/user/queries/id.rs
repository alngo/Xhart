use serde::{Serialize, Deserialize};

use crate::{domain::{abstract_repository::UserRepository, user::{Email, UserId}}, application::abstract_handler::Handler};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct IdQuery {
    email: Email
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct IdResponse {
    id: UserId
}

pub struct Information<'r, R> {
    repository: &'r R,
}

impl<'r, R> Information<'r, R>
where
    R: UserRepository,
{
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R> Handler for Information<'r, R>
where
    R: UserRepository,
{
    type Request = IdQuery;
    type Response = IdResponse;

    fn handle(&self, request: &Self::Request) -> Self::Response {
        let response = self.repository.get_by_email(&request.email).unwrap();
        IdResponse {
            id: response
        }
    }
}
