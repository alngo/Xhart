mod commands;
mod queries;

pub use commands::subscribe;
pub use queries::id;

pub enum UserCommand {
    Subscribe(subscribe::SubscribeCommand),
}

pub enum UserQuery {
    Id(id::IdQuery),
}
