use std::{future::{Ready, ready}, pin::Pin, task::Poll};
use std::future::Future;
use crate::service::ServiceRequest;

pub trait FromRequest: Clone {
    type Future: Future<Output=Result<Self, ()>>;
    fn from_request(req: &ServiceRequest) -> Self::Future;
}

impl FromRequest for String {
    type Future = Ready<Result<String, ()>>;

    fn from_request(req: &ServiceRequest) -> Self::Future {
        ready(Ok(req.0.url.clone()))
    }
}
