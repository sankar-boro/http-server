use loony_http::Response;
use crate::web;

pub async fn get_user(data: web::FormData) -> Response {
    Response::ok("Get User".to_string())
}

pub async fn delete_user(data: web::FormData) -> Response {
    Response::err("User Deleted".to_string())
}