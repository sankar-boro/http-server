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
        for service in app.services.iter() {
            let s = service.1.new_service();
            let d = s.call("Request".to_owned());
            println!("Route response: {:?}", d);
        }
        let configs = app.config.get_routes();

        for config in configs.iter() {
            for route_scope in &config.name {
                let s = route_scope.1.new_service();
                let d = s.call("param".to_string());
                println!("{}", d);
            }
        }
    }
}
