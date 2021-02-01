#![feature(type_alias_impl_trait)]

fn main() {}
struct Controller {}

impl tower_service::Service<String> for Controller {
    type Response = String;
    type Error = failure::Error;

    type Future = impl std::future::Future<Output = Result<Self::Response, Self::Error>> + Send;

    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::todo!()
    }

    fn call(&mut self, _req: String) -> Self::Future {
        async move { failure::bail!("compiler bug") }
    }
}
