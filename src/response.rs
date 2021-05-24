use crate::{DB, request::HttpRequest, service::ServiceRequest};
use crate::request::Request;
use futures::{FutureExt, executor::block_on};
use loony_service::{Service, ServiceFactory};
use std::{cell::{RefCell},rc::Rc};
use crate::resource::{CreateResourceService, ResourceService};
use ahash::AHashMap;
use crate::extensions::Extensions;

pub struct Response<'a> {
    routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>,
    extensions: Rc<Extensions>
}

impl<'a> Response<'a> {
    pub fn new(routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>, extensions: Rc<Extensions>) -> Self {
        Self {
            routes,
            extensions,
        }
    }

    pub fn build(&self, req: &Request, db: DB) -> Result<String, ()> {
        if let Some(path) = &req.uri {
            let service = self.routes.get(path);
            if let Some(s) = service {
                let sr = ServiceRequest(HttpRequest { url: String::from(path), extensions: self.extensions.clone() });
                let mut a = Rc::clone(s);
                let b = a.call(sr);
                let c = block_on(b).unwrap();
                let d = c.0.value;
                return Ok(d);
            }
        }
        Err(())
    }
}