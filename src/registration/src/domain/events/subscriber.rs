use serde::{Deserialize, Serialize};

use super::abstract_event::Event;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum NewSubscriberEvent {
    NewSubscriberSubscription {
        id: uuid::Uuid,
        email: String,
        name: String,
    },
    NewSubscriberConfirmation {
        id: uuid::Uuid,
    },
    NewSubscriberExpiration {
        id: uuid::Uuid,
    },
}

impl Event for NewSubscriberEvent {
    fn event_type(&self) -> String {
        match self {
            NewSubscriberEvent::NewSubscriberSubscription { .. } => {
                "NewSubscriberSubscription".to_string()
            }
            NewSubscriberEvent::NewSubscriberConfirmation { .. } => {
                "NewSubscriberConfirmation".to_string()
            }
            NewSubscriberEvent::NewSubscriberExpiration { .. } => {
                "NewSubscriberExpiration".to_string()
            }
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
