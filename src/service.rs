
use crate::route::Route;
use crate::responder::Responder;
// #[derive(Debug)]
pub struct ServiceConfig {
    pub routes:Vec<Route>,
}

impl ServiceConfig {

    pub fn new() -> Self {
        ServiceConfig {
            routes: Vec::new(),
        }
    }

    pub fn service(&mut self, route: Route) {
        self.routes.push(route);
    }
}

pub trait ServiceConfigFactory {
    fn get_routes(&self) -> &Vec<Route>;
}

impl ServiceConfigFactory for ServiceConfig {
    fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }
}

pub trait HttpServiceFactory {
    fn get_response(&self) -> String;
}
impl<T, R> HttpServiceFactory for T where T: Fn() -> R, R:Responder {
    fn get_response(&self) -> String {
        let data = self();
        data.respond()
    }
}
pub struct ServiceFactoryWrapper<T> {
    route: String,
    factory: Option<T>,
}

impl<T> ServiceFactoryWrapper<T> {
    pub fn new(route:&str, factory: T) -> Self {
        Self {
            route: route.to_owned(),
            factory: Some(factory),
        }
    }
}
pub trait AppServiceFactory {
    fn get_route(&self) -> String;
    fn get_response(&self) -> String;
}
impl<T> AppServiceFactory for ServiceFactoryWrapper<T>
where
    T: HttpServiceFactory,
{
    fn get_route(&self) -> String {
        self.route.clone()
    }

    fn get_response(&self) -> String {
        if let Some(f) = &self.factory {
            return f.get_response();
        }

        return String::from("");
    }
}



