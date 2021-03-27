use std::future::Future;

use crate::FromRequest;
use crate::service::{Factory, RouteNewService, Extract, Wrapper};
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

type Method = String;
type RoutePath = String;
pub struct Route {
    pub name: Vec<(RoutePath, Method, BoxedRouteNewService)>,
    pub scope: String,
}


impl<'route> Route {
    pub fn route<T, P, R, O>(mut self, route_path: &'route str, factory: (&'static str, T)) -> Self 
    where 
        T: Factory<P, R, O> + Clone + 'static, 
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder + 'static, 
    {
        
        let route = Box::new(RouteNewService::new(Extract::new(Wrapper::new(factory.1))));
        self.name.push((route_path.to_owned(), factory.0.to_owned(), route));
        self
    }
}

pub fn scope(scope: &str) -> Route {
    Route {
        scope: scope.to_string(),
        name: Vec::new(),
    }
}

pub fn get<T>(f:T) -> (&'static str, T) {
    ("GET", f)
}

pub fn post<T>(f:T) -> (&'static str, T) {
    ("POST", f)
}