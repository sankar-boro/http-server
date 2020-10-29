mod service;
mod map;
mod map_err;
mod map_init_error;
use std::pin::Pin;
use std::future::Future;

struct User;
struct UserError;
impl service::Service for User {
    type Request = String;

    type Response = Self;

    type Error = UserError;

    type Future = Pin<Box<Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        todo!()
    }
}
fn main() {

}