use std::fmt;

use serde::{de::DeserializeOwned, Serialize};

pub trait Event:
    Serialize + DeserializeOwned + Clone + PartialEq + fmt::Debug + Sync + Send
{
    fn event_type(&self) -> String;
    fn event_version(&self) -> String;
}
