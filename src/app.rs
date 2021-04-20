#![allow(dead_code)]

use std::future::Future;

use super::AppState;
use crate::{FromRequest, route::{Route, RouteService}};
use crate::service::{Factory};
use crate::responder::Responder;
use crate::extensions::Extensions;
use crate::service::{Extract, Wrapper};
use loony_service::{Service, ServiceFactory};
use crate::config::{ ServiceConfig, ServiceConfigFactory };

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

pub type RouteNewService = Box<
    dyn ServiceFactory<
            Request = String,
            Response = String,
            Service = RouteService,
            Error = (),
        >
    >;
pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
    pub services: Vec<RouteNewService>,
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

    pub fn route(mut self, route: &str, factory: Route) -> Self 
    {
        self.services.push(Box::new(factory));
        self
    }

    pub fn configure<'a, T>(mut self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig) {
        let mut configs = ServiceConfig::new();
        cnfg(&mut configs);
        self.config = Box::new(configs);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web;
    use crate::controller;
    use crate::route::Route;
    use loony_service::Service;

    async fn index(req: String) -> String {
        req
    }

    #[test]
    fn app() {
        let app = App::new()
        .configure(|cfg: &mut ServiceConfig| {
            cfg.service(
                    web::scope("/user")
                    .route("/get", Route::new().route(controller::get_user))
                );
        });

        let services = app.services;
        assert_eq!(1, services.len());
    }
}