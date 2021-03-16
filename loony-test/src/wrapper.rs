use std::marker::PhantomData;
use crate::{FromRequest, Factory, Responder};
use std::task::{Context};

pub type BoxedRouteService = Box<
    dyn Service<
        Request = String,
        Response = String,
    >,
>;

pub type BoxedRouteNewService = Box<
    dyn ServiceFactory<
        Request = String,
        Response = String,
        Service = BoxedRouteService,
    >,
>;

pub trait Service {
  type Request;
  type Response;

  fn call(&self, param: Self::Request) -> Self::Response;
}

pub trait ServiceFactory {
  type Request;
  type Response;
  type Service;

  fn new_service(&self);
}

// Structs
pub struct Wrapper<T, Arg, Res> 
where 
  T: Factory<Arg, Res>,
  Res: Responder,
{
  service: T,
  _t: PhantomData<(Arg, Res)>
}

pub struct Extract<T: FromRequest, S> {
  service: S,
  _t: PhantomData<T>
}

pub struct ExtractService<T: FromRequest, S> {
    service: S,
    _t: PhantomData<T>,
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

impl<T, Arg, Res> Wrapper<T, Arg, Res> 
where 
  T: Factory<Arg, Res>,
  Res: Responder,
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

impl<T, Arg, Res> Service for Wrapper<T, Arg, Res> 
where 
  T: Factory<Arg, Res>,
  Res: Responder,
{
  type Request = (Arg, String);
  type Response = String;
  
  fn call(&self, _t: Self::Request) -> Self::Response {
    let service = &self.service;
    let t = service.factory_call(_t.0);
    t.respond()
  }
}


impl<T: FromRequest, S> ServiceFactory for Extract<T, S> 
where S: Service<
          Request = (T, String),
          Response = String,
        >
{
    type Request = String;
    type Response = String;
    type Service = ExtractService<T, S>;

    fn new_service(&self) {
        todo!()
    }
}

impl<T> ServiceFactory for RouteNewService<T> 
where 
  T: ServiceFactory<
    Request=String,
    Response=String,
  >,
  T::Service: 'static,
{
    type Request = String;

    type Response = String;

    type Service = BoxedRouteService;

    fn new_service(&self) {
        todo!()
    }
}