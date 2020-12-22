use crate::service::{ServiceConfig, ServiceConfigFactory};
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
    pub extensions: Extensions,
    pub services: Vec<Box<dyn AppServiceFactory>>,
    config: Box<dyn ServiceConfigFactory>,
}

// #[derive(Debug)]
impl App {
    pub fn new() -> Self {
        Self {
            app_data: AppState {
                name: "".to_owned(),
            },
            extensions: Extensions::new(),
            services: Vec::new(),
            config: Box::new(ServiceConfig::new()),
        }
    }

    pub fn app_data<U: 'static>(mut self, ext: U) -> Self {
        self.extensions.insert(ext);
        self
    }

    pub fn service<T>(mut self, route: &str, factory: T) -> Self where T: HttpServiceFactory + 'static {
        self.services.push(Box::new(ServiceFactoryWrapper::new(route, factory)));
        self
    }

    pub fn configure<'a, T>(mut self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig) {
        let mut configs = ServiceConfig::new();
        cnfg(&mut configs);
        self.config = Box::new(configs);
        self
    }

    pub fn get_app_data(&self) -> &AppState {
        &self.app_data
    }

}

impl Builder for App {
    type Product = App;
}