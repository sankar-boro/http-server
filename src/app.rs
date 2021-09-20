use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use super::AppState;
use crate::app_service::AppInit;
use crate::{config::{AppService, ServiceConfig}, extensions::Extensions, resource::{Resource, ResourceService}, route::Route, service::{AppServiceFactory, HttpServiceFactory}};
use futures::executor::block_on;
use s4nk4r_service::IntoServiceFactory;

pub trait Builder {
    type Product;
}

pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
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

    pub fn data<U: 'static>(mut self, ext: U) -> Self {
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

impl IntoServiceFactory<AppInit> for App {
    fn into_factory(self) -> AppInit {
        AppInit {
            services: Rc::new(RefCell::new(self.services)),
            app_data: self.app_data,
            extensions: self.extensions,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{App, DB, web};
    use crate::controller;
    use crate::route::Route;

    async fn index(req: DB) -> String {
        String::from("")
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
        });
        // .route(web::get("/").route(index));

        let services = app.services;
        assert_eq!(3, services.len());
    }
}

