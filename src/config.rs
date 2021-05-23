use crate::{
  // DB,
  // resource::{
  //   ResourceService,
  // },
  scope::{
    Scope, BoxedResourceServiceFactory
  },
  // service::{ServiceRequest, ServiceResponse}
};
// use loony_service::ServiceFactory;


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
