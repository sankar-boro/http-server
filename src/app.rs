use crate::service::ServiceConfig;
use crate::route::Route;
pub trait Builder {
    type Product;
}
#[derive(Clone, Debug)]
pub struct App<'route> {
    name:String,
    routes:Vec<&'route str>,
    middlewares:Vec<String>,
}
impl<'buf> App<'buf> {
    pub fn new() -> Self {
        Self {
            name: "Loony".to_owned(),
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    pub fn app_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn route<T, Responder>(mut self, route: &'buf str, exec: T) -> Self where T: Fn() -> Responder {
        let res = exec();
        self.routes.push(route);
        self
    }

    pub fn service<T>(self, cnfg: T) -> Self where T: Fn(Route) {
        self
    }

    pub fn config<'a, 'b, T>(self, cnfg: T) -> Self where T: Fn(ServiceConfig<'a, 'b>) {
        cnfg(ServiceConfig {
            routes: Vec::new()
        });
        self
    }
}

impl<'buf> Builder for App<'buf> {
    type Product = App<'buf>;
}