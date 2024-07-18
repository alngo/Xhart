use serde::{Deserialize, Serialize};

use super::abstract_event::Event;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum UserEvent {
    NewUserSubscription {
        id: uuid::Uuid,
        email: String,
        name: String,
    },
    UserConfirmation {
        id: uuid::Uuid,
    },
    UserExpiration {
        id: uuid::Uuid,
    },
}

impl Event for UserEvent {
    fn event_type(&self) -> String {
        match self {
            UserEvent::NewUserSubscription { .. } => {
                "NewUserSubscription".to_string()
            }
            UserEvent::UserConfirmation { .. } => {
                "UserConfirmation".to_string()
            }
            UserEvent::UserExpiration { .. } => {
                "UserExpiration".to_string()
            }
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}
