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

pub enum Method {
  GET,
  POST,
}

type RoutePath = String;

pub struct Route {
    pub service: BoxedRouteNewService,
    pub method: Method,
}

impl<'route> Route {
    pub fn new() -> Route {
        Route {
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
        RouteService { service }
    }
}

fn method(method: Method) -> Route {
    Route::new().method(method)
}
pub fn get() -> Route {
    method(Method::GET)
}

pub fn post() -> Route {
    method(Method::POST)
}
#[cfg(test)]
mod tests {
    use super::*;

    async fn index(req: String) -> String {
        req
    }
    #[test]
    fn route() {
        let route = Route::new();
        let mut route_service = route.new_service();
        let service = route_service.call("name".to_string());
        assert_eq!("Hello World!", service);

        let route = Route::new().route(index);
        let mut route_service = route.new_service();
        let service = route_service.call("name".to_string());
        assert_eq!("name", service);
    }
}