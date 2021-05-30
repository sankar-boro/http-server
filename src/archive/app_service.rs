use futures::future::ready;
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
    type Request = ();

    type Response = ();

    type Error = ();

    type Config = ();

    type Service = AppHttpService;

    type InitError = ();

    type Future = Ready<Result<AppHttpService, ()>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let mut config = AppService::new();
        std::mem::take(&mut *self.services.borrow_mut())
        .into_iter()
        .for_each(|mut srv| srv.register(&mut config));

        let services = config.into_services();
        
        ready(Ok(AppHttpService {
            services,
        }))
    }
}
 
pub struct AppHttpService {
    pub(crate) services: Vec<Rc<RefCell<ResourceService>>>
}

impl Service for AppHttpService {
    type Request = ();

    type Response = ();

    type Error = ();

    type Future = Ready<Result<(), ()>>;

    fn call(&mut self, req: Self::Request) -> Self::Future {
        ready(Ok(()))
    }
}
