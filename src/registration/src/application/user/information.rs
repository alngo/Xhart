use serde::{Serialize, Deserialize};

use crate::{domain::abstract_repository::UserRepository, application::abstract_handler::Handler};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct InformationQuery {
    id: uuid::Uuid
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct InformationResponse {
    id: uuid::Uuid,
    username: String,
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
    type Request = InformationQuery;
    type Response = InformationResponse;

    fn handle(&self, request: &Self::Request) -> Self::Response {
        let response = self.repository.get_by_id(request.id).unwrap();
        InformationResponse {
            id: response.id,
            username: response.username,
        }
    }
}
