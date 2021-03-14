use futures::Future;

use crate::{responder::Responder, service::{HttpServiceFactory, HttpServiceFactoryWrapper}};
use crate::service::{Factory};

// #[derive(Debug)]
pub struct Route {
    pub name: Vec<(String, Box<dyn HttpServiceFactory>)>,
    pub scope: String,
}

impl<'route> Route {
    pub fn route<T, R, O: 'static>(mut self, route: (&'route str, T)) -> Self 
    where 
        T: Factory<R, O>, 
        // P: FromRequest,
        R: Future<Output=O> + 'static, 
        O: Responder, 
    {
        let s = Box::new(HttpServiceFactoryWrapper::new(route.1));
        self.name.push((route.0.to_string(), s));
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

    // pub fn route<T>(mut self, route: &str, factory: T) -> Self where T: HttpServiceFactory + 'static {
#[derive(Debug)]
pub struct RouteService<T> where T: HttpServiceFactory + 'static {
    route: String,
    serve: T
}