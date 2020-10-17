use crate::service::ServiceConfig;
use crate::route::Route;
use crate::service::HttpServiceFactory;
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

    pub fn route<T>(mut self, route: &'buf str, factory: T) -> Self where T: HttpServiceFactory + 'static {
        self.routes.push(route);
        self
    }

    pub fn service<T>(self, cnfg: T) -> Self where T: Fn(Route) {
        self
    }

    pub fn config<'a, 'b, T>(self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig<'a, 'b>) {
        let mut configs = ServiceConfig {
            routes: Vec::new()
        };
        cnfg(&mut configs);
        // println!("Service Config: {:?}", configs.routes);
        self
    }
}

impl<'buf> Builder for App<'buf> {
    type Product = App<'buf>;
}