// use loony_http;
use crate::{App, AppState};
use std::any::{Any, TypeId};

// pub type AppInstance = Box<dyn Fn() -> App + Send + Sync + 'static>;
pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { app: Box::new(app) }
    }

    pub fn run(&self) {
        let app = (self.app)();
        println!("{:?}", app.extensions.get::<AppState>());
        for service in app.services.iter() {
            println!("Route name: {} | Route response: {}", service.get_route(), service.get_response());
        }
        let configs = app.config.get_routes();

        for config in configs.iter() {
            println!("Route scope: {:?} | Scoped Routes: {:?}", config.get_scope(), config.get_scope_routes());
        }
    }
}
