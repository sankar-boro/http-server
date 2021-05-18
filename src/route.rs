use std::future::Future;
use loony_service::{
    Service,
    ServiceFactory
};

use crate::{FromRequest};
use crate::service::{Factory,Extract, RouteServiceFactory, Wrapper};
use crate::responder::Responder;
use crate::default::default;
use crate::DB;

#[derive(Clone)]
pub enum Method {
  GET,
  POST,
}

pub type BoxedRouteService = Box<
    dyn Service<
        Request = DB,
        Response = String,
        Error = (),
    >,
>;

pub type BoxedRouteServiceFactory = Box<
    dyn ServiceFactory<
            Request = DB,
            Response = String,
            Service = BoxedRouteService,
            Error = (),
        >
    >;

pub struct Route {
    pub path: String,
    pub service: BoxedRouteServiceFactory,
    pub method: Method,
}

impl<'route> Route {
    pub fn new(path: &str) -> Route {
        Route {
            path: path.to_owned(),
            service: Box::new(RouteServiceFactory::new(Extract::new(Wrapper::new(default)))),
            method: Method::GET,
        }
    }

    pub fn route<T, P, R, O>(mut self, factory: T) -> Self 
    where 
        T: Factory<P, R, O> + Clone + 'static, 
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder + 'static, 
    {
        
        let service = Box::new(RouteServiceFactory::new(Extract::new(Wrapper::new(factory))));
        self.service = service;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
}

pub type BoxService = Box<dyn Service<Request = DB, Response = String, Error = ()>>;

pub struct RouteService {
    service: BoxService,
    method: Method,
}

impl Service for RouteService {
    type Request = DB;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
        self.service.call(req)
    }
}

impl ServiceFactory for Route {
    type Request = DB;
    type Response = String;
    type Error = ();
    type Service = RouteService;

    fn new_service(&self) -> Self::Service {
        let service = self.service.new_service();
        RouteService { service, method: self.method.clone() }
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
#[cfg(test)]
mod tests {
    use super::*;

    async fn index(req: String) -> String {
        req
    }
    #[test]
    fn route() {
    }
}