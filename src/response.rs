use crate::{DB, request::HttpRequest, service::ServiceRequest};
use crate::request::Request;
use futures::{FutureExt, executor::block_on};
use loony_service::{Service, ServiceFactory};
use std::{cell::{RefCell},rc::Rc};
use crate::resource::CreateResourceService;
use ahash::AHashMap;
use crate::extensions::Extensions;

pub struct Response<'a> {
    routes: &'a AHashMap<String, Rc<RefCell<CreateResourceService>>>,
    extensions: Rc<Extensions>
}

impl<'a> Response<'a> {
    pub fn new(routes: &'a AHashMap<String, Rc<RefCell<CreateResourceService>>>, extensions: Rc<Extensions>) -> Self {
        Self {
            routes,
            extensions,
        }
    }

    pub fn build(&self, req: &Request, db: DB) -> Result<String, ()> {
        if let Some(path) = req.path {
            let service = self.routes.get(path);
            if let Some(s) = service {
                let sr = ServiceRequest(HttpRequest { url: String::from(path), extensions: self.extensions.clone() });
                let a = Rc::clone(s);
                let b = &mut *a.borrow_mut();
                let c = block_on(b);
                let mut d = c.unwrap();
                let e = d.call(sr);
                let f = block_on(e).unwrap();
                let g = f.0.value;
                return Ok(g);
            }
        }
        // Err(())
        Ok("".to_string())
    }
}