#![allow(dead_code)]

use std::future::Future;

use super::AppState;
use crate::{FromRequest, service::RouteNewService};
use crate::service::{Factory};
use crate::responder::Responder;
use crate::extensions::Extensions;
use crate::service::{Extract, ServiceConfig, Wrapper, ServiceConfigFactory, AppServiceFactory};
use loony_service::{Service, ServiceFactory};

pub trait Builder {
    type Product;
}

pub type BoxedRouteService = Box<
    dyn Service<
        Request = String,
        Response = String,
        Error = (),
    >,
>;

pub type BoxedRouteNewService = Box<
    dyn ServiceFactory<
            Request = String,
            Response = String,
            Service = BoxedRouteService,
            Error = (),
        >
    >;

pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
    pub services: Vec<(String, BoxedRouteNewService)>,
    pub config: Box<dyn ServiceConfigFactory>,
}

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

    pub fn route<T, Arg, R, O>(mut self, route: (&str, T)) -> Self 
    where 
        T: Factory<Arg, R, O> + Clone + 'static, 
        Arg: FromRequest + 'static, 
        R: Future<Output=O>+ 'static, 
        O: Responder + 'static
    {
        let wrapper = Wrapper::new(route.1);
        let extract = Extract::new(wrapper);
        let factory = Box::new(RouteNewService::new(extract));
        self.services.push((route.0.to_owned(), factory));
        self
    }

    // pub fn service<T, P, R>(mut self, route: &str, factory: T) -> Self 
    // where 
    //     T: Factory<P, R> + Clone + 'static, 
    //     P: FromRequest + 'static, 
    //     // R: Future<Output=O> + 'static, 
    //     R: Responder
    // {
    //     self.services.push((route.to_string(), Box::new(HttpServiceFactoryWrapper::new(factory))));
    //     self
    // }

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