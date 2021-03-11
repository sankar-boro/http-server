use {
  std::{
    sync::Arc,
    rc::Rc,
    cell::RefCell,
    future::Future,
    task::{Context, Poll},
  },
};

pub trait Service<Req> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
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


impl<S, Req> Service<Req> for RefCell<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.borrow_mut().poll_ready(ctx)
    }

    fn call(&mut self, request: Req) -> S::Future {
        self.borrow_mut().call(request)
    }
}

impl<S, Req> Service<Req> for Rc<RefCell<S>>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.borrow_mut().poll_ready(ctx)
    }

    fn call(&mut self, request: Req) -> S::Future {
        (&mut (**self).borrow_mut()).call(request)
    }
}

impl<S, Req> ServiceFactory<Req> for Rc<S>
where
    S: ServiceFactory<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Config = S::Config;
    type Service = S::Service;
    type InitError = S::InitError;
    type Future = S::Future;

    fn new_service(&self, cfg: S::Config) -> S::Future {
        self.as_ref().new_service(cfg)
    }
}

impl<S, Req> ServiceFactory<Req> for Arc<S>
where
    S: ServiceFactory<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Config = S::Config;
    type Service = S::Service;
    type InitError = S::InitError;
    type Future = S::Future;

    fn new_service(&self, cfg: S::Config) -> S::Future {
        self.as_ref().new_service(cfg)
    }
}

/// Trait for types that can be converted to a `Service`
pub trait IntoService<S, Req>
where
    S: Service<Req>,
{
    /// Convert to a `Service`
    fn into_service(self) -> S;
}

/// Trait for types that can be converted to a `ServiceFactory`
pub trait IntoServiceFactory<SF, Req>
where
    SF: ServiceFactory<Req>,
{
    /// Convert `Self` to a `ServiceFactory`
    fn into_factory(self) -> SF;
}

impl<S, Req> IntoService<S, Req> for S
where
    S: Service<Req>,
{
    fn into_service(self) -> S {
        self
    }
}

impl<SF, Req> IntoServiceFactory<SF, Req> for SF
where
    SF: ServiceFactory<Req>,
{
    fn into_factory(self) -> SF {
        self
    }
}

/// Convert object of type `U` to a service `S`
pub fn into_service<I, S, Req>(tp: I) -> S
where
    I: IntoService<S, Req>,
    S: Service<Req>,
{
    tp.into_service()
}
