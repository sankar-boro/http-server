use crate::service::HttpServiceFactory;
// #[derive(Debug)]
pub struct Route {
    name: Vec<(String, Box<dyn Fn() + 'static>)>,
    scope: String,
}

impl<'route> Route<> {
    pub fn route<T>(mut self, route: (&'route str, T)) -> Self where T: Fn() + 'static {
        self.name.push((route.0.to_owned(), Box::new(route.1)));
        self
    }

    pub fn get_scope(&self) -> &str {   
        &self.scope
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