use responder::Responder;
use loony_http::Response;

use crate::responder;
use crate::web;

pub struct User{}

pub fn get_user(_: web::FormData<User>, _: String) -> impl Responder {
    println!("Get user");
    let res = Response::from(String::from("Loony"));
    res
}

pub fn delete_user() -> impl Responder {
    println!("Delete user");
    let res = Response::from(String::from("Loony"));
    res
}