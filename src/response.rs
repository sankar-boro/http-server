use crate::DB;
use httparse::Request;
use loony_service::Service;
use std::{cell::{RefCell},rc::Rc};
use crate::resource::ResourceService;
use ahash::AHashMap;

pub struct Response<'a> {
    routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>
}

impl<'a> Response<'a> {
    pub fn new(routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>) -> Self {
        Self {
            routes,
        }
    }

    pub fn build(&self, req: &Request, db: DB) -> Result<String, ()> {
        if let Some(path) = req.path {
            let service = self.routes.get(path);
            if let Some(s) = service {
                let mut s = s.borrow_mut();
                return Ok(s.call(db));
            }
        }
        Err(())
    }
}