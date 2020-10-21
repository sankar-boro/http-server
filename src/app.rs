use crate::service::ServiceConfig;
use crate::route::Route;
use crate::service::HttpServiceFactory;
use super::AppState;
use crate::extensions::Extensions;

pub trait Builder {
    type Product;
}
#[derive(Debug)]
pub struct App<'route> {
    app_data:AppState,
    extensions: Extensions,
    routes:Vec<&'route str>,
}
impl<'buf> App<'buf> {
    pub fn new() -> Self {
        Self {
            app_data: AppState {
                name: "".to_owned(),
            },
            routes: Vec::new(),
            extensions: Extensions::new()
        }
    }

    pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
        self.extensions.insert(ext);
        self
    }

    pub fn route<T>(mut self, route: &'buf str, factory: T) -> Self where T: HttpServiceFactory + 'static {
        self.routes.push(route);
        self
    }

    pub fn service<'a, 'b, T>(self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig<'a, 'b>) {
        let mut configs = ServiceConfig {
            routes: Vec::new()
        };
        cnfg(&mut configs);
        self
    }
}

impl<'buf> Builder for App<'buf> {
    type Product = App<'buf>;
}