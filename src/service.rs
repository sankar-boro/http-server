use async_std::task; 
use std::future::Future;
use std::marker::PhantomData;

use crate::FromRequest;
use crate::route::Route;
use crate::responder::Responder;
use crate::route::{BoxedRouteService};


pub trait Factory<Arg, Res, O>: Clone + 'static 
where 
  Res: Future<Output=O>, 
  O: Responder
{
  fn factory_call(&self, param: Arg) -> Res;
}

pub struct ServiceConfig {
  pub routes:Vec<Route>,
}

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

pub trait ServiceConfigFactory {
  fn get_routes(&self) -> &Vec<Route>;
}

impl ServiceConfigFactory for ServiceConfig {
  fn get_routes(&self) -> &Vec<Route> {
    &self.routes
  }
}

pub trait Service {
  type Request;
  type Response;

  fn call(&self, param: Self::Request) -> Self::Response;
}

pub trait ServiceFactory {
  type Request;
  type Response;
  type Service: Service<Request=Self::Request, Response=Self::Response>;

  fn new_service(&self) -> Self::Service;
}

impl<T, A, Res, O> Factory<(A,), Res, O> for T 
where 
  T: Fn(A,) -> Res + Clone + 'static, 
  Res: Future<Output=O>,
  O: Responder,
{
  fn factory_call(&self, (one,): (A,)) -> Res {
    (self)(one)
  }
}

impl<T, A, B, Res, O> Factory<(A,B,), Res, O> for T 
where 
  T: Fn(A, B,) -> Res + Clone + 'static, 
  Res: Future<Output=O>,
  O: Responder,
{
  fn factory_call(&self, (one, two,): (A, B,)) -> Res {
    (self)(one, two)
  }
}

// Structs
pub struct Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
  service: T,
  _t: PhantomData<(Arg, Res, O)>
}

pub struct Extract<T: FromRequest, S> {
  service: S,
  _t: PhantomData<T>
}

pub struct ExtractService<T: FromRequest, S> {
    service: S,
    _t: PhantomData<T>,
}

struct RouteServiceWrapper<T: Service> {
    service: T,
}

pub struct RouteNewService<T>
where
  T: ServiceFactory<
    Request=String
  >,
  T::Service: 'static,
{
  service: T,
}
/**
* Implementations
*
*/


// Struct Implementation

impl<T, Arg, Res, O> Clone for Wrapper<T, Arg, Res, O>
where
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
    fn clone(&self) -> Self {
      Self {
        service: self.service.clone(),
        _t: PhantomData,
      }
    }
}

impl<T, Arg, Res, O> Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
  {
    // service: Fn(Arg) -> Res
    pub fn new(service: T) -> Self {
      Self {
        service,
        _t: PhantomData,
      }  
  }
}

impl<T: FromRequest, S> Extract<T, S> {
  pub fn new(service: S) -> Self {
    Self {
      service,
      _t: PhantomData,
    }
  }  
}

impl<T> RouteNewService<T>
where
  T: ServiceFactory<
    Request=String,
    Response=String,
  >,
  T::Service: 'static,
{
  pub fn new(service: T) -> Self {
    Self {
      service,
    }
  } 
}


// Trait Implementation

impl<T, Arg, Res, O> Service for Wrapper<T, Arg, Res, O> 
where 
  T: Factory<Arg, Res, O>,
  Res: Future<Output=O>,
  O: Responder,
{
  type Request = Arg;
  type Response = String;
  
  fn call(&self, param: Self::Request) -> Self::Response {
    let t = self.service.factory_call(param);
    let r = task::block_on(t);
    r.respond()
  }
}

impl<T: FromRequest, S> Service for ExtractService<T, S>
where
    S: Service<
            Request = T,
            Response = String,
        > + Clone,
{
    type Request = String;
    type Response = String;

    fn call(&self, req: Self::Request) -> Self::Response {
      let t = T::from_request(req.clone());
      let b = self.service.call(t);
      b
    }
}


impl<T: FromRequest, S> ServiceFactory for Extract<T, S> 
where S: Service<
          Request = T,
          Response = String,
        > + Clone,
{
    type Request = String;
    type Response = String;
    type Service = ExtractService<T, S>;

    fn new_service(&self) -> Self::Service {
      ExtractService {
        service: self.service.clone(),
        _t: PhantomData,
      }
    }
}

impl<T> Service for RouteServiceWrapper<T>
where
    T: Service<
        Request = String,
        Response = String,
    >,
{
    type Request = String;
    type Response = String;

    fn call(&self, req: Self::Request) -> Self::Response {
      let a = &self.service;
      let b = a.call(req);
      b
    }
}

impl Service for BoxedRouteService {
    type Request = String;
    type Response = String;

    fn call(&self, param: Self::Request) -> Self::Response {
      (**self).call(param.clone())
    }
}


impl<T> ServiceFactory for RouteNewService<T> 
where 
  T: ServiceFactory<
    Request=String,
    Response=String,
  >,
  T::Service: Service + 'static,
{
    type Request = String;

    type Response = String;

    type Service = BoxedRouteService;

    fn new_service(&self) -> Self::Service {
      let s = &self.service;
      let service = s.new_service();
      let d = Box::new(RouteServiceWrapper {
        service,
      });
      d
    }
}