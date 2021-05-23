use std::pin::Pin;
use std::future::Future;

use loony_service::{Service, ServiceFactory};
use crate::service::{ServiceRequest, ServiceResponse};

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