use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

pub trait Service {
    type Request;
    type Response;
    type Error;
    // type Future: Future<Output = Result<Self::Response, Self::Error>>;
    fn call(&mut self, req: Self::Request) -> Self::Response;
}

pub trait ServiceFactory {
    type Request;
    type Response;
    type Error;
    // type Config;
    type Service: Service<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::Error,
    >;
    // type Future: Future<Output = Result<Self::Service, Self::InitError>>;
    fn new_service(&self) -> Self::Service;
}

impl<'a, S> Service for &'a mut S
where
    S: Service + 'a,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    // type Future = S::Future;

    fn call(&mut self, request: Self::Request) -> S::Response {
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
    // type Future = S::Future;

    fn call(&mut self, request: Self::Request) -> S::Response {
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
    // type Future = S::Future;

    fn call(&mut self, request: Self::Request) -> S::Response {
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
    // type Future = S::Future;

    fn call(&mut self, request: Self::Request) -> S::Response {
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
    // type Config = S::Config;
    type Service = S::Service;
    // type InitError = S::InitError;
    // type Future = S::Future;

    fn new_service(&self) -> S::Service {
        self.as_ref().new_service()
    }
}

impl<S> ServiceFactory for Arc<S>
where
    S: ServiceFactory,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    // type Config = S::Config;
    type Service = S::Service;
    // type InitError = S::InitError;
    // type Future = S::Future;

    fn new_service(&self) -> S::Service {
        self.as_ref().new_service()
    }
}

pub trait IntoService<T>
where
    T: Service,
{
    fn into_service(self) -> T;
}

pub trait IntoServiceFactory<T>
where
    T: ServiceFactory,
{
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

pub fn into_service<T, S>(tp: T) -> S
where
    S: Service,
    T: IntoService<S>,
{
    tp.into_service()
}

/// `BoxedServiceFactory` is a Factory that creates new `Service` Object in the heap.
/// `Service` helps transform `Request` to `Response`;
pub type BoxedServiceFactory<S, Req, Res, Err> = Box<dyn ServiceFactory<Request=Req, Response=Res, Error=Err, Service=S>>;