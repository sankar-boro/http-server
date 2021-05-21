use std::{future::ready, pin::Pin, task::Poll};
use std::future::Future;
use std::marker::PhantomData;
use crate::service::{Service, ServiceFactory};

pub trait Responder {}
// ******************************************************************************
pub trait Factory<P, R, O>: Clone + 'static {
    fn call(&self, param: P) -> R;
}

impl<T, P, R, O> Factory<P, R, O> for T 
where
    T: Fn(P) -> R + Clone + 'static,
    R: Future<Output=O>,
    O: Responder 
{
    fn call(&self, param: P) -> R {
        (self)(param)
    }
}
// ******************************************************************************

struct Handler<T, P, R, O> 
where 
    T: Factory<P, R, O>
{
    factory: T, 
    _t: PhantomData<(P, R, O)>
}
// ******************************************************************************
type BoxedRouteService = Box<
    dyn Service<
        Request=String,
        Response=String,
        Error=(),
        Future=Pin<Box<dyn Future<Output=Result<String, ()>>>>
    >
>;
// ******************************************************************************
struct RouteHandlerService {}
impl Service for RouteHandlerService {
    type Request = String;
    type Response = String;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<String, ()>>>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let a = req;
        let b = Ok(a);
        let c = ready(b);
        let d = Box::pin(c);
        d
    }
}
// ******************************************************************************
struct RouteHandlerServiceFactory {}
impl ServiceFactory for RouteHandlerServiceFactory {
    type Request = String;
    type Response = String;
    type Config = ();
    type Error = ();
    type InitError = ();
    type Service = BoxedRouteService;
    type Future = Pin<Box<dyn Future<Output=Result<BoxedRouteService, ()>>>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let a: BoxedRouteService = Box::new(RouteHandlerService {});
        let b = Ok(a);
        let c = ready(b);
        let d = Box::pin(c);
        d
    }
}
// ******************************************************************************