use crate::{
  resource::{
    ResourceService,
  },
  scope::{
    Scope,
  }
};
use loony_service::ServiceFactory;

type BoxedResourceServiceFactory = Box<
    dyn ServiceFactory<
        Request = String, 
        Response = String, 
        Error = (), 
        Service = ResourceService
    >
>;

pub struct ServiceConfig {
  pub services:Vec<BoxedResourceServiceFactory>,
}

impl ServiceConfig {
  pub fn new() -> Self {
    ServiceConfig {
      services: Vec::new(),
    }
  }
	
	pub fn service(&mut self, service: Scope) {
    let services = service.services;
    self.services.extend(services);
  }
}
