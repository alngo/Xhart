use serde::{Deserialize, Serialize};

use crate::domain::{abstract_repository::Repository, subscriber::Subscriber};

pub trait CommandHandler<T, R> {
    fn handle(&self, command: T) -> R;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    email: String,
    username: String,
}

pub struct TSubscribe<'r, R> {
    repository: &'r R,
}

impl<'r, R> TSubscribe<'r, R>
where
    R: Repository,
{
    pub fn new(repository: &'r R) -> Self {
        Self { repository }
    }
}

impl<'r, R> CommandHandler<Subscribe, uuid::Uuid> for TSubscribe<'r, R>
where
    R: Repository,
{
    fn handle(&self, command: Subscribe) -> uuid::Uuid {
        let subscriber = Subscriber::subscribe(&command.username, &command.email, self.repository);
        self.repository.create(subscriber.unwrap()).unwrap()
    }
}
