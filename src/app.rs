use crate::service::ServiceConfig;
use crate::route::RouteService;
use crate::service::{HttpServiceFactory, AppServiceFactory, ServiceFactoryWrapper};
use super::AppState;
use crate::extensions::Extensions;

pub trait Builder {
    type Product;
}
// #[derive(Debug)]
pub struct App {
    app_data:AppState,
    extensions: Extensions,
    pub services: Vec<Box<dyn AppServiceFactory>>,
}
impl App {
    pub fn new() -> Self {
        Self {
            app_data: AppState {
                name: "".to_owned(),
            },
            extensions: Extensions::new(),
            services: Vec::new(),
        }
    }

    pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
        self.extensions.insert(ext);
        self
    }

    pub fn service<T>(mut self, route: &str, factory: T) -> Self where T: HttpServiceFactory + 'static {
        self.services.push(Box::new(ServiceFactoryWrapper::new(factory)));
        self
    }

    pub fn configure<'a, 'b, T>(self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig<'a, 'b>) {
        let mut configs = ServiceConfig {
            routes: Vec::new()
        };
        cnfg(&mut configs);
        self
    }
}

impl Builder for App {
    type Product = App;
}