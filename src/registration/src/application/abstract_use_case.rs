pub trait UseCase {
    type Request;
    type Response;

    fn execute(&self, request: &Self::Request) -> Self::Response;
}
