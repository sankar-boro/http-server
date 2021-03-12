use futures::Future;
use crate::{route::Route};
use crate::responder::Responder;
use std::marker::PhantomData;
use async_std::task;
use crate::Request;

pub struct ServiceConfig {
  pub routes:Vec<Route>,
}
pub trait ServiceConfigFactory {
  fn get_routes(&self) -> &Vec<Route>;
}
pub trait HttpServiceFactory {
  fn service_call(&self) -> String;
}

#[derive(Clone)]
pub(crate) struct HttpServiceFactoryWrapper<T, R, O> {
  factory: T,
  _t: PhantomData<(R, O)>
}

/**
  ......................................
  Implementations
  ......................................
*/

impl ServiceConfig {
  pub fn new() -> Self {
    ServiceConfig {
      routes: Vec::new(),
    }
  }
	
	pub fn service(&mut self, route: Route) {
    self.routes.push(route);
  }
}

impl ServiceConfigFactory for ServiceConfig {
  fn get_routes(&self) -> &Vec<Route> {
    &self.routes
  }
}

impl<T, R, O> HttpServiceFactoryWrapper<T, R, O> 
where 
  T: Factory<R, O>, 
  R: Future<Output=O>, 
  O: Responder
{
  pub fn new(factory: T) -> Self {
    Self {
      factory,
      _t: PhantomData
    }
  }
}

impl<T, R, O> HttpServiceFactory for HttpServiceFactoryWrapper<T, R, O> 
where 
  T: Factory<R, O>, 
  R: Future<Output=O>, 
  O: Responder 
{
  fn service_call(&self) -> String {
    let factory = &self.factory;
    let f_res = task::block_on(factory.call());
    f_res.respond()
  }
}


/**
  ......................................
  This is totally something else
  ......................................
*/
// P = Parameters
// R = Returned Response
// O = Future Output type
pub trait Factory<R, O>: Clone + 'static
where
  // P: FromRequest,
  R: Future<Output=O>,
  O: Responder,
{
  fn call(&self) -> R;
}

impl<T, R, O> Factory<R, O> for T 
where 
  T: Fn(Request) -> R + Clone + 'static, 
  // P: FromRequest, 
  R: Future<Output=O>, 
  O: Responder 
{
  fn call(&self) -> R {
    (self)(Request{
      method: String::from("GET"),
      version: String::from("http/1.1"),
      url: String::from("/get"),
    })
  }
}
