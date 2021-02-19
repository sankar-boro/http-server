
use std::future::Future;
use std::task::{self, Context, Poll};
pub trait Service<Req> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn poll_ready(&mut self, ctx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Req) -> Self::Future;
}

pub trait ServiceFactory<Req> {
    type Response;
    type Error;
    type Config;
    type Service: Service<Req, Response = Self::Response, Error = Self::Error>;
    type InitError;
    type Future: Future<Output = Result<Self::Service, Self::InitError>>;
    fn new_service(&self, cfg: Self::Config) -> Self::Future;
}


impl<'a, S, Req> Service<Req> for &'a mut S
where
    S: Service<Req> + 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&mut self, request: Req) -> S::Future {
        (**self).call(request)
    }
}