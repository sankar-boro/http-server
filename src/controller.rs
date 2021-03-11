// use responder::Responder;
// use loony_http::Response;

use crate::responder;
use crate::Request;

pub async fn get_user(request: Request) -> String {
    String::from("Get user")
}

pub async fn delete_user(request: Request) -> String {
    String::from("User Deleted")
}