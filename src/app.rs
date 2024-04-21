use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use super::AppState;
use crate::app_service::AppInit;
use crate::{
    route,
    config::{AppService, ServiceConfig}, 
    extensions::Extensions, 
    resource::{Resource, ResourceService}, 
    route::Route, service::{AppServiceFactory, HttpServiceFactory}
};
use futures::executor::block_on;
use loony_service::IntoServiceFactory;

#[derive(Clone)]
pub struct Data<T>(pub T);

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
            extensions: RefCell::new(Some(self.extensions)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{App, DB};
    use crate::controller;
    use crate::route::Route;

    async fn index(_: DB) -> String {
        String::from("")
    }

    #[test]
    fn app() {
        let app = App::new()
        .configure(|cfg: &mut ServiceConfig| {
            cfg.service(
                    route::scope("/user")
                    .route(Route::new("/user").to(controller::get_user))
                    .route(Route::new("/delete").to(controller::get_user))
                );
        });
        // .route(web::get("/").route(index));

        let services = app.services;
        assert_eq!(3, services.len());
    }
}

