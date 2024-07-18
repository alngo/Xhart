pub trait Handler {
    type Command;
    type Response;

    fn handle(&self, command: &Self::Command) -> Self::Response;
}
