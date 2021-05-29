use futures::{Future, future::Ready};
use loony_service::{ServiceFactory, Service};
use std::rc::Rc;
use std::cell::RefCell;

use crate::AppState;
use crate::extensions::Extensions;
use crate::service::{AppServiceFactory, ServiceRequest, ServiceResponse};

pub struct AppInit {
    pub services: Rc<RefCell<Vec<Box<dyn AppServiceFactory>>>>,
    pub app_data: AppState,
    pub extensions: Extensions,
}

impl ServiceFactory for AppInit {
    type Request = ServiceRequest;

    type Response = ServiceResponse;

    type Error = ();

    type Config = ();

    type Service = AppService;

    type InitError = ();

    type Future = AppFutureService;

    fn new_service(&self, cfg: Self::Config) -> Self::Future {
        AppFutureService {}
    }
}

pub struct AppService {}
impl Service for AppService {
    type Request = ServiceRequest;

    type Response = ServiceResponse;

    type Error = ();

    type Future = Ready<Result<ServiceResponse, ()>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        todo!()
    }
}

pub struct AppFutureService {}

impl Future for AppFutureService {
    type Output = Result<AppService, ()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}