use crate::{App};
use crate::builder::Builder;

pub type AppInstance = Box<dyn Fn() -> App + 'static>;

pub struct HttpServer {
    app: AppInstance,
    builder: Builder,
}

impl HttpServer {
    pub fn new<F: Fn() -> App + 'static>(app: F) -> Self {
        Self { 
            app: Box::new(app), 
            builder: Builder::new() 
        }
    }

    pub fn run(&self) {
        let app = (self.app)();
        self.builder.run();
    }
}