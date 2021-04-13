use std::future::Future;

use crate::FromRequest;
use crate::service::{Factory};
use crate::{responder::Responder, service::{Service, ServiceFactory}};

pub type BoxedRouteService = Box<
    dyn Service<
        Request = String,
        Response = String,
    >,
>;

pub type BoxedRouteNewService = Box<
    dyn ServiceFactory<
        Request = String,
        Response = String,
        Service = BoxedRouteService,
    >,
>;

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
        // Route {
        //     service: Box::new(RouteNewService::new(Extract::new(Wrapper::new(|data: String| async {
        //         String::from("")
        //     })))),
        //     method: Method::GET,
        // }
        unimplemented!()
    }

    pub fn route<T, P, R, O>(mut self, factory: T) -> Self 
    where 
        T: Factory<P, R, O> + Clone + 'static, 
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder + 'static, 
    {
        
        // let service = Box::new(RouteNewService::new(Extract::new(Wrapper::new(factory))));
        // self.service = service;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
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