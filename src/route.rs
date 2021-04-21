use std::future::Future;
use loony_service::{
    Service,
    ServiceFactory
};

use crate::{FromRequest};
use crate::service::{Factory,Extract, RouteNewService, Wrapper};
use crate::responder::Responder;
use crate::app::{BoxedRouteNewService};
use crate::default::default;

#[derive(Clone)]
pub enum Method {
  GET,
  POST,
}

type RoutePath = String;

pub struct Route {
    pub path: String,
    pub service: BoxedRouteNewService,
    pub method: Method,
}

impl<'route> Route {
    pub fn new(path: &str) -> Route {
        Route {
            path: path.to_owned(),
            service: Box::new(RouteNewService::new(Extract::new(Wrapper::new(default)))),
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
        
        let service = Box::new(RouteNewService::new(Extract::new(Wrapper::new(factory))));
        self.service = service;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
}

pub type BoxService = Box<dyn Service<Request = String, Response = String, Error = ()>>;

pub struct RouteService {
    service: BoxService,
    method: Method,
}

impl Service for RouteService {
    type Request = String;
    type Response = String;
    type Error = ();

    fn call(&mut self, req: Self::Request) -> Self::Response {
        self.service.call(req)
    }
}

impl ServiceFactory for Route {
    type Request = String;
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
        let route = Route::new("/get");
        let mut route_service = route.new_service();
        let service = route_service.call("name".to_string());
        assert_eq!("Hello World!", service);

        let route = Route::new("/delete").route(index);
        let mut route_service = route.new_service();
        let service = route_service.call("name".to_string());
        assert_eq!("name", service);
    }
}