use std::rc::Rc;
use std::pin::Pin;
use std::future::Future;
use loony_service::{Service, ServiceFactory};
use crate::extensions::Extensions;

/**
*
*/
#[derive(Clone)]
pub struct HttpRequest {
    pub url: String,
    pub extensions: Rc<Extensions>
}
#[derive(Clone)]
pub struct HttpResponse {
    pub value: String,
}
/**
*
*/
#[derive(Clone)]
pub struct ServiceRequest(pub HttpRequest);

#[derive(Clone)]
pub struct ServiceResponse(pub HttpResponse);

/**
*
*/
pub type BoxedRouteService = Box<
    dyn Service<
        Request=ServiceRequest,
        Response=ServiceResponse,
        Error=(),
        Future=Pin<Box<dyn Future<Output=Result<ServiceResponse, ()>>>>
    >
>;

pub type BoxedRouteServiceFactory = Box<
    dyn ServiceFactory<
        Request=ServiceRequest,
        Response=ServiceResponse,
        Error=(),
        Service=BoxedRouteService,
        Future=Pin<Box<dyn Future<Output=Result<BoxedRouteService, ()>>>>,
        Config=(),
        InitError=()
    >
>;

pub type HttpService = BoxedRouteService;

pub type HttpNewService = BoxedRouteServiceFactory;