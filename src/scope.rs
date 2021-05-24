use async_std::task::block_on;
use loony_service::{ServiceFactory};
use crate::resource::{Resource, ResourceService};
use crate::service::{ServiceRequest, ServiceResponse};
use crate::{resource::CreateResourceService, route::{Route}};
use crate::http::AppServiceFactory;

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
    pub factory_services: Vec<ResourceService>,
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
            factory_services: Vec::new(),
        }
    }

    pub fn route(mut self, route: Route) -> Self {
        self.services.push(Box::new(Resource::new(self.scope.clone()).route(route)));
        self
    }
}

impl AppServiceFactory for Scope {
  fn register(&mut self) {
    let a = &self.services;
    for s in a.iter() {
        let b = s.new_service(());
        let c = block_on(b).unwrap();
        self.factory_services.push(c);
    }   
  }
}