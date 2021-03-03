use crate::service::{HttpServiceFactoryWrapper, ServiceConfig, ServiceConfigFactory};
use crate::route::RouteService;
use crate::service::{HttpServiceFactory, AppServiceFactory, ServiceFactoryWrapper, HttpAppServiceFactory};
use super::AppState;
use crate::extensions::Extensions;

pub trait Builder {
    type Product;
}
// #[derive(Debug)]
pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
    pub services: Vec<(String ,Box<dyn HttpAppServiceFactory>)>,
    pub config: Box<dyn ServiceConfigFactory>,
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

    pub fn route<T>(mut self, route: (&str, T)) -> Self where T: HttpServiceFactory + 'static {
        self.services.push((route.0.to_string(), Box::new(HttpServiceFactoryWrapper::new(route.1))));
        self
    }

    pub fn service<T>(mut self, route: &str, factory: T) -> Self where T: HttpServiceFactory + 'static {
        self.services.push((route.to_string(), Box::new(HttpServiceFactoryWrapper::new(factory))));
        self
    }

    pub fn configure<'a, T>(mut self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig) {
        let mut configs = ServiceConfig::new();
        cnfg(&mut configs);
        self.config = Box::new(configs);
        self
    }

}

impl Builder for App {
    type Product = App;
}