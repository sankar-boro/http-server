use crate::{
    resource::{
        Resource, 
        ResourceService, 
    },
    route::Route};
use loony_service::{ServiceFactory};

pub trait ScopeFactory{
    fn register(&mut self);
}
pub struct Scope {
    scope: String,
    pub services: Vec<Box<dyn ServiceFactory<Request = String, Response = String, Error = (), Service=ResourceService>>>,
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
        }
    }

    pub fn route(mut self, path: &str, route: Route) -> Self {
        let a = Resource::new(path, &self.scope).route(route);
        self.services.push(Box::new(a));
        self
    }
}
