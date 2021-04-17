use crate::{
    resource::{
        HttpServiceFactory, 
        Resource, 
        ResourceService
    },
    route::Route};
use loony_service::ServiceFactory;

pub trait ScopeFactory{
    fn register(&self);
}
pub struct Scope {
    scope: String,
    services: Vec<Box<dyn ServiceFactory<Request = String, Response = String, Error = (), Service=ResourceService>>>,
    factory_ref: Vec<String>
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
            factory_ref: Vec::new(),
        }
    }

    pub fn route(mut self, path: &str, route: Route) -> Self {
        let a = Resource::new(path).route(route);
        // self.services.push(Box::new());
        self
    }
}

impl ScopeFactory for Scope {
    fn register(&self) {
        let _service = self.services.iter().map(|service| {
            let a = service.new_service();
        });
    }
}