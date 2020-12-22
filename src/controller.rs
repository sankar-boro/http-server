use responder::Responder;
use loony_http::Response;

use crate::responder;

pub fn get_user() -> impl Responder {
    let res = Response::from(String::from("Loony"));
    res
}

pub fn delete_user() -> impl Responder {
    let res = Response::from(String::from("Loony"));
    res
}