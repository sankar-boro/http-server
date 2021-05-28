use super::AppState;
use crate::{config::ServiceConfig, extensions::Extensions, resource::{Resource, ResourceService}, route::Route, service::{AppServiceFactory, HttpServiceFactory}};
use futures::executor::block_on;

pub trait Builder {
    type Product;
}

pub struct App {
    app_data:AppState,
    pub extensions: Extensions,
    pub services: Vec<Box<dyn AppServiceFactory>>,
    pub factories: Option<Vec<ResourceService>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_data: AppState {
                name: "".to_owned(),
            },
            extensions: Extensions::new(),
            services: Vec::new(),
            factories: None,
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

impl AppServiceFactory for App {
    fn register(&mut self) {
        // let mut factories = Vec::new();
        // let resource_services = &self.services;
        // for resource_service in resource_services.iter() {
        //     let resource_service = resource_service.new_service(());
        //     let a = block_on(resource_service).unwrap();
        //     factories.push(a);
        // }

        // self.factories = Some(factories);
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

