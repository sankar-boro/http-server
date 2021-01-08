use crate::{responder::Responder, service::HttpServiceFactory};
use crate::service::Factory;
// #[derive(Debug)]
pub struct Route {
    // name: Vec<(String, Box<dyn HttpServiceFactory>)>,
    name: Vec<String>,
    pub scope: String,
}

impl<'route> Route<> {
    pub fn route<T, I, R>(mut self, route: (&'route str, T)) -> Self where T: Factory<I, R> + 'static, R: Responder {
        // self.name.push((route.0.to_owned(), Box::new(route.1)));
        self.name.push(route.0.to_owned());
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
        //     _route.push_str(", Route response:");
        //     let data = &route.1;
        //     let data = data.get_response();
        //     _route.push_str(&data);
        // }
        _route
    }
}

pub fn scope(scope: &str) -> Route {
    Route{
        scope: scope.to_owned(),
        name:Vec::new(),
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