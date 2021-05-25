use crate::scope::{Scope, BoxedResourceServiceFactory};

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
