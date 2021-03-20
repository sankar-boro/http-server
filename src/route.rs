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
pub struct Route {
    pub name: Vec<(String, BoxedRouteNewService)>,
    pub scope: String,
}

impl<'route> Route {
    pub fn route<T, P, R, O>(mut self, scope: &'route str, factory: T) -> Self 
    where 
        T: Factory<P, R, O> + Clone + 'static, 
        P: FromRequest + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder + 'static, 
    {
        
        let route = Box::new(RouteNewService::new(Extract::new(Wrapper::new(factory))));
        self.name.push((scope.to_owned(), route));
        self
    }

    pub fn get_scope(&self) -> &str {   
        &self.scope
    }

    pub fn get_scope_routes(&self) -> String {
        let mut _route = String::from("");
        // for route in self.name.iter() {
        //     _route.push_str(" Route name:");
        //     _route.push_str(&route.0);
        //     println!("Response: {}", route.1.call());
        // }
        _route
    }
}

pub fn scope(scope: &str) -> Route {
    Route{
        scope: scope.to_string(),
        name: Vec::new(),
    }
}

pub fn get<T>(route: &str, get:T) -> (&str, T) where T: Fn() {
    (route, get)
}
