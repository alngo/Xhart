pub trait Handler {
    type Request;
    type Response;

    fn handle(&self, request: &Self::Request) -> Self::Response;
}
