use crate::scope::{
  Scope,
};

pub struct ServiceConfig {
  pub services:Vec<Scope>,
}

impl ServiceConfig {
  pub fn new() -> Self {
    ServiceConfig {
      services: Vec::new(),
    }
  }
	
	pub fn service(&mut self, service: Scope) {
    self.services.push(service);
  }
}

pub trait ServiceConfigFactory {
  fn get_services(&self) -> &Vec<Scope>;
}

impl ServiceConfigFactory for ServiceConfig {
  fn get_services(&self) -> &Vec<Scope> {
    &self.services
  }
}