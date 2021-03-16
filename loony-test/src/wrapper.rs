use std::marker::PhantomData;
use crate::{FromRequest, Factory, Responder};

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
  type Service: Service<Request=Self::Request, Response=Self::Response>;

  fn new_service(&self) -> Self::Service;
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

impl<T, Arg, Res> Clone for Wrapper<T, Arg, Res>
where
  T: Factory<Arg, Res>,
  Res: Responder,
{
    fn clone(&self) -> Self {
      Self {
        service: self.service.clone(),
        _t: PhantomData,
      }
    }
}

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

impl<T: FromRequest, S> Service for ExtractService<T, S>
where
    S: Service<
            Request = (T, String),
            Response = String,
        > + Clone,
{
    type Request = String;
    type Response = String;

    fn call(&self, req: Self::Request) -> Self::Response {
      req
    }
}


impl<T: FromRequest, S> ServiceFactory for Extract<T, S> 
where S: Service<
          Request = (T, String),
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
      req
    }
}

impl Service for BoxedRouteService {
    type Request = String;

    type Response = String;

    fn call(&self, param: Self::Request) -> Self::Response {
      param
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