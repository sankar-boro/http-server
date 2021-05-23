use crate::DB;
use crate::request::Request;
use loony_service::Service;
use std::{cell::{RefCell},rc::Rc};
use crate::resource::CreateResourceService;
use ahash::AHashMap;

pub struct Response<'a> {
    routes: &'a AHashMap<String, Rc<RefCell<CreateResourceService>>>
}

impl<'a> Response<'a> {
    pub fn new(routes: &'a AHashMap<String, Rc<RefCell<CreateResourceService>>>) -> Self {
        Self {
            routes,
        }
    }

    pub fn build(&self, req: &Request, db: DB) -> Result<String, ()> {
        if let Some(path) = req.path {
            let service = self.routes.get(path);
            if let Some(s) = service {
                return Ok("".to_string());
                // return Ok(s.call(db));
            }
        }
        Err(())
    }
}