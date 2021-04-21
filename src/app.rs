#![allow(dead_code)]

use super::AppState;
use crate::{route::{Route, RouteService}};
use crate::extensions::Extensions;
use loony_service::{Service, ServiceFactory};
use crate::config::{ ServiceConfig };
use crate::resource::{Resource, ResourceService};

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
            // Service = RouteService,
            Service = ResourceService,
            Error = (),
        >
    >;
pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
    pub services: Vec<RouteNewService>,
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

    pub fn route(mut self, route: Route) -> Self 
    {
        self.services.push(Box::new(Resource::new("".to_string()).route(route)));
        self
    }

    pub fn configure<'a, T>(mut self, cnfg: T) -> Self where T: Fn(&mut ServiceConfig) {
        let mut configs = ServiceConfig::new();
        cnfg(&mut configs);
        self.services.extend(configs.services);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web;
    use crate::controller;
    use crate::route::Route;

    async fn index(req: String) -> String {
        req
    }

    #[test]
    fn app() {
        let app = App::new()
        .configure(|cfg: &mut ServiceConfig| {
            cfg.service(
                    web::scope("/user")
                    .route(Route::new("/user").route(controller::get_user))
                    .route(Route::new("/delete").route(controller::get_user))
                );
        }).route(web::get("/").route(index));

        let services = app.services;
        assert_eq!(3, services.len());
    }
}

