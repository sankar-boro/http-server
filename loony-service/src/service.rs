use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{self, Context, Poll};
use crate::map::Map;
use crate::map_err::MapErr;
use crate::map_init_error::MapInitErr;

pub trait Service {
    /// Requests handled by the service.
    type Request;

    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// The future response value.
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Returns `Ready` when the service is able to process requests.
    ///
    /// If the service is at capacity, then `Pending` is returned and the task
    /// is notified when the service becomes ready again. This function is
    /// expected to be called while on a task.
    ///
    /// This is a **best effort** implementation. False positives are permitted.
    /// It is permitted for the service to return `Ready` from a `poll_ready`
    /// call and the next invocation of `call` results in an error.
    ///
    /// # Notes
    /// 1. `.poll_ready()` might be called on different task from actual service call.
    /// 1. In case of chained services, `.poll_ready()` get called for all services at once.
    fn poll_ready(&mut self, ctx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Process the request and return the response asynchronously.
    ///
    /// This function is expected to be callable off task. As such,
    /// implementations should take care to not call `poll_ready`. If the
    /// service is at capacity and the request is unable to be handled, the
    /// returned `Future` should resolve to an error.
    ///
    /// Calling `call` without calling `poll_ready` is permitted. The
    /// implementation must be resilient to this fact.
    fn call(&mut self, req: Self::Request) -> Self::Future;

    /// Map this service's output to a different type, returning a new service
    /// of the resulting type.
    ///
    /// This function is similar to the `Option::map` or `Iterator::map` where
    /// it will change the type of the underlying service.
    ///
    /// Note that this function consumes the receiving service and returns a
    /// wrapped version of it, similar to the existing `map` methods in the
    /// standard library.
    fn map<F, R>(self, f: F) -> Map<Self, F, R>
    where
        Self: Sized,
        F: FnMut(Self::Response) -> R,
    {
        Map::new(self, f)
    }

    /// Map this service's error to a different error, returning a new service.
    ///
    /// This function is similar to the `Result::map_err` where it will change
    /// the error type of the underlying service. For example, this can be useful to
    /// ensure that services have the same error type.
    ///
    /// Note that this function consumes the receiving service and returns a
    /// wrapped version of it.
    fn map_err<F, E>(self, f: F) -> MapErr<Self, F, E>
    where
        Self: Sized,
        F: Fn(Self::Error) -> E,
    {
        MapErr::new(self, f)
    }
}

/// Factory for creating `Service`s.
///
/// Acts as a service factory. This is useful for cases where new `Service`s
/// must be produced. One case is a TCP server listener. The listener
/// accepts new TCP streams, obtains a new `Service` using the
/// `ServiceFactory` trait, and uses the new `Service` to process inbound
/// requests on that new TCP stream.
///
/// `Config` is a service factory configuration type.
pub trait ServiceFactory {
    /// Requests handled by the created services.
    type Request;

    /// Responses given by the created services.
    type Response;

    /// Errors produced by the created services.
    type Error;

    /// Service factory configuration.
    type Config;

    /// The kind of `Service` created by this factory.
    type Service: Service<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;

    /// Errors potentially raised while building a service.
    type InitError;

    /// The future of the `Service` instance.
    type Future: Future<Output = Result<Self::Service, Self::InitError>>;

    /// Create and return a new service asynchronously.
    fn new_service(&self, cfg: Self::Config) -> Self::Future;

    /// Map this service's output to a different type, returning a new service
    /// of the resulting type.
    fn map<F, R>(self, f: F) -> crate::map::MapServiceFactory<Self, F, R>
    where
        Self: Sized,
        F: FnMut(Self::Response) -> R + Clone,
    {
        crate::map::MapServiceFactory::new(self, f)
    }

    /// Map this service's error to a different error, returning a new service.
    fn map_err<F, E>(self, f: F) -> crate::map_err::MapErrServiceFactory<Self, F, E>
    where
        Self: Sized,
        F: Fn(Self::Error) -> E + Clone,
    {
        crate::map_err::MapErrServiceFactory::new(self, f)
    }

    /// Map this factory's init error to a different error, returning a new service.
    fn map_init_err<F, E>(self, f: F) -> MapInitErr<Self, F, E>
    where
        Self: Sized,
        F: Fn(Self::InitError) -> E + Clone,
    {
        MapInitErr::new(self, f)
    }
}

impl<'a, S> Service for &'a mut S
where
    S: Service + 'a,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&mut self, request: Self::Request) -> S::Future {
        (**self).call(request)
    }
}

impl<S> Service for Box<S>
where
    S: Service + ?Sized,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), S::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&mut self, request: Self::Request) -> S::Future {
        (**self).call(request)
    }
}

impl<S> Service for RefCell<S>
where
    S: Service,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.borrow_mut().poll_ready(ctx)
    }

    fn call(&mut self, request: Self::Request) -> S::Future {
        self.borrow_mut().call(request)
    }
}

impl<S> Service for Rc<RefCell<S>>
where
    S: Service,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.borrow_mut().poll_ready(ctx)
    }

    fn call(&mut self, request: Self::Request) -> S::Future {
        (&mut (**self).borrow_mut()).call(request)
    }
}

impl<S> ServiceFactory for Rc<S>
where
    S: ServiceFactory,
{
    type Request = S::Request;
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

impl<S> ServiceFactory for Arc<S>
where
    S: ServiceFactory,
{
    type Request = S::Request;
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
pub trait IntoService<T>
where
    T: Service,
{
    /// Convert to a `Service`
    fn into_service(self) -> T;
}

/// Trait for types that can be converted to a `ServiceFactory`
pub trait IntoServiceFactory<T>
where
    T: ServiceFactory,
{
    /// Convert `Self` to a `ServiceFactory`
    fn into_factory(self) -> T;
}

impl<T> IntoService<T> for T
where
    T: Service,
{
    fn into_service(self) -> T {
        self
    }
}

impl<T> IntoServiceFactory<T> for T
where
    T: ServiceFactory,
{
    fn into_factory(self) -> T {
        self
    }
}

/// Convert object of type `T` to a service `S`
pub fn into_service<T, S>(tp: T) -> S
where
    S: Service,
    T: IntoService<S>,
{
    tp.into_service()
}
