use std::{
    future::Future,  
    task::{Context, Poll},
    pin::Pin
};
use s4nk4r_service::{
    Service,
    ServiceFactory
};
use crate::{
    scope::Scope,
    FromRequest,
    default::default,
    responder::Responder,
    service::{ServiceRequest, ServiceResponse},
    handler::{Factory,Extract, RouteServiceFactory, Handler},
};

#[derive(Clone)]
pub enum Method {
  GET,
  POST,
}

pub type BoxedRouteService = Box<
    dyn Service<
        Request=ServiceRequest,
        Response=ServiceResponse,
        Error=(),
        Future=Pin<Box<dyn Future<Output=Result<ServiceResponse, ()>>>>
    >
>;

pub type BoxedRouteServiceFactory = Box<
    dyn ServiceFactory<
        Request=ServiceRequest,
        Response=ServiceResponse,
        Error=(),
        Service=BoxedRouteService,
        Future=Pin<Box<dyn Future<Output=Result<BoxedRouteService, ()>>>>,
        Config=(),
        InitError=()
    >
>;


pub type BoxService = Pin<
    Box<
        dyn Future<Output=Result<BoxedRouteService, ()>>
    >
>;

// #[derive(Clone)]
pub struct Route {
    pub path: String,
    pub service: BoxedRouteServiceFactory,
    pub method: Method,
}

impl<'route> Route {
    pub fn new(path: &str) -> Route {
        Route {
            path: path.to_owned(),
            service: Box::new(RouteServiceFactory::new(Extract::new(Handler::new(default)))),
            method: Method::GET,
        }
    }

    pub fn to<T, P, R, O>(mut self, factory: T) -> Self 
    where 
        T: Factory<P, R, O> + Clone + 'static, 
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder + 'static, 
    {
        
        let service = Box::new(RouteServiceFactory::new(Extract::new(Handler::new(factory))));
        self.service = service;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
}

#[pin_project::pin_project]
pub struct RouteFutureService {
    #[pin]
    pub fut: BoxService,
    pub method: Method,
}

impl Future for RouteFutureService {
    type Output = Result<RouteService, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx)? {
            Poll::Ready(service) => Poll::Ready(Ok(RouteService {
                service,
                method: this.method.clone(),
            })),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub struct RouteService {
    service: BoxedRouteService,
    method: Method,
}

impl Service for RouteService {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, ()>>>>;

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        self.service.call(req)
    }
}

impl ServiceFactory for Route {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = ();
    type Service = RouteService;
    type InitError = ();
    type Config = ();
    type Future = RouteFutureService;

    fn new_service(&self, _: ()) -> Self::Future {
        let fut = self.service.new_service(());
        RouteFutureService { fut, method: self.method.clone() }
    }
}

fn method(path: &str, method: Method) -> Route {
    Route::new(path).method(method)
}

pub fn get(path: &str) -> Route {
    method(path, Method::GET)
}

pub fn post(path: &str) -> Route {
    method(path, Method::POST)
}

pub fn scope(scope: &str) -> Scope {
  Scope::new(scope)
}

#[cfg(test)]
mod tests {
    use futures::{FutureExt, executor::block_on};
    use crate::request::HttpRequest;
    use super::*;

    async fn index(_: String) -> String {
        "Hello World!".to_string()
    }

    #[test]
    fn route() {
        // let sr = ServiceRequest(HttpRequest { url: "/home".to_string(), extensions: &Extensions::new() });
        // let r = Route::new("/home");
        // let r = r.route(index);
        // let a = r.new_service(());
        // let mut b = block_on(a).unwrap();
        // let c = b.call(sr);
        // let d = block_on(c).unwrap();
        // let e = d.0.value;
        // assert_eq!("Hello World!".to_string(), e);
    }
}