use std::future::Future;
use std::marker::PhantomData;
use pin_project::pin_project;
use futures_util::{FutureExt, ready as _ready};
use std::{future::{Ready, ready}, pin::Pin, task::Poll};

use crate::responder::Responder;
use crate::service::BoxedRouteService;
use crate::extract::FromRequest;
use loony_service::{Service, ServiceFactory};
use crate::service::{ServiceRequest, ServiceResponse};

// ******************************************************************************
pub trait Factory<P, R, O>: Clone + 'static 
where 
    R: Future<Output=O>, 
    O: Responder,
{
    fn call(&self, param: P) -> R;
}
// ******************************************************************************

pub struct Handler<T, P, R, O> 
where
    T: Factory<P, R, O>,
    R: Future<Output=O>,
    O: Responder,
{
    factory: T, 
    _t: PhantomData<(P, R, O)>
}

impl<T, P, R, O> Handler<T, P, R, O> 
where
    T: Factory<P, R, O>,
    R: Future<Output=O>,
    O: Responder,
{
    pub fn new(factory: T) -> Self {
        Handler {
            factory,
            _t: PhantomData,
        }
    }
}

impl<T, P, R, O> Clone for Handler<T, P, R, O> 
where
    T: Factory<P, R, O>,
    R: Future<Output=O>,
    O: Responder,
{
    fn clone(&self) -> Self {
        Handler {
            factory: self.factory.clone(),
            _t: PhantomData,
        }
    }
}

impl<T, P, R, O> Service for Handler<T, P, R, O> 
where 
    T: Factory<P, R, O>,
    R: Future<Output=O>,
    O: Responder,
{
    type Request = (P, ServiceRequest);
    type Response = ServiceResponse;
    type Error = ();
    type Future = HandlerServiceResponse<R, O>;

    fn call(&mut self, (param, req): (P, ServiceRequest)) -> Self::Future {
        let a = &self.factory;
        let b = a.call(param);
        HandlerServiceResponse {
            fut: b,
            fut2: None,
            req: Some(req),
        }
    }
}
// ******************************************************************************
#[pin_project]
pub struct HandlerServiceResponse<R, O> 
where
    R: Future<Output = O>,
    O: Responder
{
    #[pin]
    fut: R,
    #[pin]
    fut2: Option<O::Future>,
    req: Option<ServiceRequest>
}
impl<R, O> Future for HandlerServiceResponse<R, O> 
where 
    R: Future<Output = O>,
    O: Responder,
{
    type Output = Result<ServiceResponse, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        if let Some(fut) = this.fut2.as_pin_mut() {
            return match fut.poll(cx) {
                Poll::Ready(res) => {
                    Poll::Ready(Ok(res))
                }
                Poll::Pending => Poll::Pending,
            };
        }
        match this.fut.poll(cx) {
            Poll::Ready(res) => {
                let fut = res.respond(this.req.as_ref().unwrap());
                self.as_mut().project().fut2.set(Some(fut));
                self.poll(cx)
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

// ******************************************************************************
pub struct Extract<T: FromRequest, S> {
    service: S,
    _t: PhantomData<T>
}

impl<T: FromRequest, S> Extract<T, S> {
    pub fn new(service: S) -> Self {
        Extract {
            service,
            _t: PhantomData,
        }
    }
}

impl<T: FromRequest, S> ServiceFactory for Extract<T, S>
where 
    S: Service<
        Request=(T, ServiceRequest),
        Response=ServiceResponse,
        Error=()
    > + Clone
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Config = ();
    type Service = ExtractService<T, S>;
    type InitError = ();
    type Future = Ready<Result<Self::Service, ()>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let a= ExtractService {
            service: self.service.clone(),
            _t: PhantomData,
        };
        ready(Ok(a))
    }
}
// ******************************************************************************
pub struct ExtractService<T, S> {
    service: S,
    _t: PhantomData<T>
}

impl<T: FromRequest, S> Service for ExtractService<T, S> 
where 
    S: Service<
        Request=(T, ServiceRequest),
        Response=ServiceResponse,
        Error=()
    > + Clone
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Future = ExtractResponse<T, S>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        ExtractResponse {
            req: req.clone(),
            service: self.service.clone(),
            fut: T::from_request(&req),
            fut_s: None,
        }
    }
}
// ******************************************************************************
#[pin_project]
pub struct ExtractResponse <T: FromRequest, S: Service> {
    req: ServiceRequest,
    service: S,
    #[pin]
    fut: T::Future,
    #[pin]
    fut_s: Option<S::Future>,
}

impl<T: FromRequest, S: Service> Future for ExtractResponse<T, S> 
where
    S: Service<
        Request = (T, ServiceRequest),
        Response = ServiceResponse,
        Error=()
    >,
{
    type Output = Result<ServiceResponse, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        if let Some(fut) = this.fut_s.as_pin_mut() {
            return match fut.poll(cx) {
                Poll::Ready(res) => {
                    Poll::Ready(res)
                }
                Poll::Pending => Poll::Pending,
            };
        }

        match _ready!(this.fut.poll(cx)) {
            Err(_) => {
                Poll::Ready(Err(()))
            }
            Ok(data) => {
                let l = this.service.call((data, this.req.clone()));
                self.as_mut().project().fut_s.set(Some(l));
                self.poll(cx)
            }
        }
    }
}

// ******************************************************************************
struct RouteHandlerService<T: Service> {
    factory:T 
}

impl<T> Service for RouteHandlerService<T> 
where
    T::Future: 'static,
    T: Service<
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = (),
    >,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<ServiceResponse, ()>>>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let a = &mut self.factory;
        Box::pin(a.call(req))
    }
}
// ******************************************************************************
pub struct RouteServiceFactory<T> 
where
    T: ServiceFactory<Request = ServiceRequest>
{
    factory: T,
}

impl<T> RouteServiceFactory<T> 
where
    T: ServiceFactory<Request = ServiceRequest>
{
    pub fn new(factory: T) -> Self {
        RouteServiceFactory {
            factory,
        }
    }
}
// ******************************************************************************
impl<T> ServiceFactory for RouteServiceFactory<T> 
where
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = ()
    >,
    T::Future: 'static,
    T::Service: 'static,
    <T::Service as Service>::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Config = ();
    type Error = ();
    type InitError = ();
    type Service = BoxedRouteService;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Service, ()>>>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let s = self.factory.new_service(())
        .map(|result| match result {
            Ok(res) => {
                let service: BoxedRouteService =
                        Box::new(RouteHandlerService { factory: res });
                    Ok(service)
            }
            Err(_) => Err(()),
        }).boxed_local();
        s
    }
}

/**
* Factory implementations
*
*/
impl<T, R, O> Factory<(), R, O> for T 
where
    T: Fn() -> R + Clone + 'static,
    R: Future<Output=O>,
    O: Responder 
{
    fn call(&self, _: ()) -> R {
        (self)()
    }
}

/// FromRequest trait impl for tuples
macro_rules! factory_tuple ({ $(($n:tt, $T:ident)),+} => {
    impl<Func, $($T,)+ Res, O> Factory<($($T,)+), Res, O> for Func
    where Func: Fn($($T,)+) -> Res + Clone + 'static,
          Res: Future<Output = O>,
          O: Responder,
    {
        fn call(&self, param: ($($T,)+)) -> Res {
            (self)($(param.$n,)+)
        }
    }
});

#[rustfmt::skip]
mod m {
    use super::*;
    factory_tuple!((0, A));
    factory_tuple!((0, A), (1, B));
    factory_tuple!((0, A), (1, B), (2, C));
    factory_tuple!((0, A), (1, B), (2, C), (3, D));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
    factory_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));
}