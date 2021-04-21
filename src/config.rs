use crate::{
  app::RouteNewService, 
  resource::{
    Resource, 
    ResourceService,
  },
  scope::{
    Scope,
  }
};
use loony_service::ServiceFactory;


type ScopeFactory = Box<
    dyn ServiceFactory<
        Request = String, 
        Response = String, 
        Error = (), 
        // Service= RouteService
        Service = ResourceService
    >
>;

pub struct ServiceConfig {
  pub services:Vec<ScopeFactory>,
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
