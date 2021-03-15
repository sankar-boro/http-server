use futures::Future;

use crate::{responder::Responder, service::{HttpServiceFactory, HttpServiceFactoryWrapper}};
use crate::service::{Factory};
use crate::web::{FormData, FormDataExtractor};
use loony_http::Response;

// #[derive(Debug)]
pub struct Route {
    pub name: Vec<(String, Box<dyn HttpServiceFactory<Request=FormData, Response=dyn Future<Output=Response>>>)>,
    pub scope: String,
}

impl<'route> Route {
    pub fn route<T, P, R, O: 'static>(mut self, scope: &'route str, factory: T) -> Self 
    where 
        T: Factory<P, R, O>, 
        P: FormDataExtractor + 'static,
        R: Future<Output=O> + 'static, 
        O: Responder, 
    {
        let s = Box::new(HttpServiceFactoryWrapper::new(|param: FormData| async {
            Response::ok(String::from("Hello"))
        }));
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