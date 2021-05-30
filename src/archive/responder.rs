use std::future::{Ready, ready, Future};
use crate::service::{HttpResponse, ServiceRequest, ServiceResponse};

pub trait Responder {
    type Future: Future<Output=ServiceResponse>;
    fn respond(&self, req: &ServiceRequest) -> Self::Future;
}

impl Responder for String {
    type Future = Ready<ServiceResponse>;

    fn respond(&self, _: &ServiceRequest) -> Self::Future {
        let r = ServiceResponse(HttpResponse{value:self.clone()});
        ready(r)
    }
}