use std::{future::{Ready, ready,}, pin::Pin, task::Poll};
use std::future::Future;
use std::marker::PhantomData;
use crate::service::{Service, ServiceFactory};
use pin_project::pin_project;

pub trait FromRequest: Clone {
    // type Future;
    fn from_request(param: &String) -> Self;
}

impl FromRequest for String {
    // type Future = String;

    fn from_request(param: &String) -> Self {
        param.clone()
    }
}

pub trait Responder {
    type Future: Future<Output=String>;
    fn respond(&self, req: &String) -> Self::Future;
}

impl Responder for String {
    type Future = Ready<String>;

    fn respond(&self, req: &String) -> Self::Future {
        ready(req.clone())
    }
}
// ******************************************************************************
pub trait Factory<P, R, O>: Clone + 'static 
where 
    R: Future<Output=O>, 
    O: Responder,
{
    fn call(&self, param: P) -> R;
}

impl<T, P, R, O> Factory<P, R, O> for T 
where
    T: Fn(P) -> R + Clone + 'static,
    P: FromRequest,
    R: Future<Output=O>,
    O: Responder 
{
    fn call(&self, param: P) -> R {
        (self)(param)
    }
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
    type Request = (P, String);

    type Response = String;

    type Error = ();

    type Future = HandlerServiceResponse<R, O>;

    fn call(&mut self, (param, req): (P, String)) -> Self::Future {
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
    req: Option<String>
}
impl<R, O> Future for HandlerServiceResponse<R, O> 
where 
    R: Future<Output = O>,
    O: Responder,
{
    type Output = Result<String, ()>;

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
pub type BoxedRouteService = Box<
    dyn Service<
        Request=String,
        Response=String,
        Error=(),
        Future=Pin<Box<dyn Future<Output=Result<String, ()>>>>
    >
>;

pub type BoxedRouteServiceFactory = Box<
    dyn ServiceFactory<
        Request=String,
        Response=String,
        Error=(),
        Service=BoxedRouteService,
        Future=Pin<Box<dyn Future<Output=Result<BoxedRouteService, ()>>>>,
        Config=(),
        InitError=()
    >
>;
// ******************************************************************************
struct RouteHandlerService<T> {
    factory:T 
}

impl<T> Service for RouteHandlerService<T> {
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
pub struct RouteHandlerServiceFactory<T> 
where
    T: ServiceFactory<Request = String>
{
    factory: T,
}

impl<T> RouteHandlerServiceFactory<T> 
where
    T: ServiceFactory<Request = String>
{
    pub fn new(factory: T) -> Self {
        RouteHandlerServiceFactory {
            factory,
        }
    }
}
// ******************************************************************************
impl<T> ServiceFactory for RouteHandlerServiceFactory<T> 
where
    T: ServiceFactory<
        Config = (),
        Request = String,
        Response = String,
        Error = (),
    >,
    T::Future: 'static,
    T::Service: 'static,
    <T::Service as Service>::Future: 'static,
{
    type Request = String;
    type Response = String;
    type Config = ();
    type Error = ();
    type InitError = ();
    type Service = BoxedRouteService;
    type Future = Pin<Box<dyn Future<Output=Result<BoxedRouteService, ()>>>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let s = self.factory.new_service(());
        let a: BoxedRouteService = Box::new(RouteHandlerService {
            factory: s,
        });
        let b = Ok(a);
        let c = ready(b);
        let d = Box::pin(c);
        d
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
        Request=(T, String),
        Response=String,
        Error=()
    > + Clone
{
    type Request = String;

    type Response = String;

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
        Request=(T, String),
        Response=String,
        Error=()
    > + Clone
{
    type Request = String;

    type Response = String;

    type Error = ();

    type Future = ExtractResponse<T, S>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let a = "".to_string();
        ExtractResponse {
            req,
            service: self.service.clone(),
            fut: T::from_request(&a),
            fut_s: None,
        }
    }
}
// ******************************************************************************
#[pin_project]
pub struct ExtractResponse <T: FromRequest, S: Service> {
    req: String,
    service: S,
    fut: T,
    #[pin]
    fut_s: Option<S::Future>,
}

impl<T: FromRequest, S: Service> Future for ExtractResponse<T, S> 
where
    S: Service<
        Request = (T, String),
        Response = String,
        Error=()
    >,
{
    type Output = Result<String, ()>;

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

        let a = this.fut.clone();
        let b = this.service.call((a, this.req.clone()));
        self.as_mut().project().fut_s.set(Some(b));
        self.poll(cx)
    }
}