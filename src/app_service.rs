use futures::{Future, future::Ready};
use loony_service::{ServiceFactory, Service};
use std::rc::Rc;
use std::cell::RefCell;
use std::task::Poll;

use crate::AppState;
use crate::config::AppService;
use crate::extensions::Extensions;
use crate::resource::Resource;
use crate::resource::ResourceService;
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

    type Service = AppHttpService;

    type InitError = ();

    type Future = AppFutureService;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let mut config = AppService::new();
        std::mem::take(&mut *self.services.borrow_mut())
        .into_iter()
        .for_each(|mut srv| srv.register(&mut config));

        let services = config.into_services();
        
        AppFutureService {
            services,
        }
    }
}
 
pub struct AppHttpService {
    services: Vec<ResourceService>
}

impl Service for AppHttpService {
    type Request = ServiceRequest;

    type Response = ServiceResponse;

    type Error = ();

    type Future = Ready<Result<ServiceResponse, ()>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        todo!()
    }
}

#[pin_project::pin_project]
pub struct AppFutureService {
    services: Vec<ResourceService>
}

impl Future for AppFutureService {
    type Output = Result<AppHttpService, ()>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}