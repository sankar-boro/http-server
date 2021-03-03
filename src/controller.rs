use responder::Responder;
use loony_http::Response;

use crate::responder;
use crate::Request;

pub fn get_user(request: Request) -> impl Responder {
    let res = Response::from(String::from("Get user"));
    res
}

pub fn delete_user(request: Request) -> impl Responder {
    let res = Response::from(String::from("User Deleted"));
    res
}