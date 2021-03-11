use futures::Future;
use crate::{route::Route, web};
use crate::responder::Responder;
use std::marker::PhantomData;
use crate::Request;
use async_std::{prelude::FutureExt, task::{self, block_on}};
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
pub(crate) struct HttpServiceFactoryWrapper<T, I, R> {
  factory: T,
  _t: PhantomData<(I, R)>
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

impl<T, I, R> HttpServiceFactoryWrapper<T, I, R> where T: Factory<I, R>, R: Future<Output=String>{
  pub fn new(factory: T) -> Self {
    Self {
      factory,
      _t: PhantomData
    }
  }
}

impl<T, I, R> HttpServiceFactory for HttpServiceFactoryWrapper<T, I, R> where T: Factory<I, R>, R: Future<Output=String>{
  fn service_call(&self) -> String {
    let factory = &self.factory;
    task::block_on(factory.call()) 
  }
}


/**
  ......................................
  This is totally something else
  ......................................
*/
pub trait Factory<T, R>
where
  R: Future<Output = String>
{
  fn call(&self) -> R;
}

impl<T, R> Factory<T, R> for T where T: Fn(Request) -> R, R: Future<Output=String>{
  fn call(&self) -> R {
    (self)(Request{})
  }
}
