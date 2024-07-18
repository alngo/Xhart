mod subscribe;
mod information;

pub enum UserCommand {
    Subscribe(subscribe::SubscribeCommand),
}

pub enum UserQuery {
    Information(information::InformationQuery),
}
