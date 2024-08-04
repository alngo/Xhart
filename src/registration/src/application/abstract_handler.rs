pub trait Handler {
    type Request;
    type Response;

    fn handle(&mut self, request: &Self::Request) -> Self::Response;
}
