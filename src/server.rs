use crate::{App, AppState};

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
        // for service in app.services.iter() {
        //     println!("Route response: {:?}", service.1.service_call());
        // }
        // let configs = app.config.get_routes();

        // for config in configs.iter() {
        //     for route_scope in &config.name {
        //         println!("Route name: {}", route_scope.0);
        //         let s = route_scope.1.service_call();
        //         println!("{:?}", s);
        //     }
        //     println!("Route scope: {:?} | Scoped Routes: {:?}", config.get_scope(), config.get_scope_routes());
        // }
    }
}
