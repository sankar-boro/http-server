use loony_service::{ServiceFactory};
use crate::resource::{Resource, ResourceService};
use crate::service::{ServiceRequest, ServiceResponse};
use crate::{resource::CreateResourceService, route::{Route}};

pub type BoxedResourceServiceFactory = Box<
    dyn ServiceFactory<
        Request = ServiceRequest, 
        Response = ServiceResponse, 
        Error = (), 
        Service = ResourceService,
        Config=(),
        InitError=(),
        Future = CreateResourceService
    >
>;

pub struct Scope {
    pub scope: String,
    pub services: Vec<BoxedResourceServiceFactory>,
    pub f_services: Vec<String>,
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
            f_services: Vec::new(),
        }
    }

    pub fn route(mut self, route: Route) -> Self {
        self.services.push(Box::new(Resource::new(self.scope.clone()).route(route)));
        self
    }
}


#[cfg(test)]
mod tests {

    async fn index(req: String) -> String {
        req
    }

    #[test]
    fn scope() {
    }
}