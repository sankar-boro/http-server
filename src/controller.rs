use loony_http::Response;
use crate::Request;

pub async fn get_user(request: Request) -> Response {
    println!("{:?}", request);
    Response::ok("Get User".to_string())
}

pub async fn delete_user(request: Request) -> Response {
    println!("{:?}", request);
    Response::err("User Deleted".to_string())
}