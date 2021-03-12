use loony_http::Response;

pub async fn get_user() -> Response {
    Response::ok("Get User".to_string())
}

pub async fn delete_user() -> Response {
    Response::err("User Deleted".to_string())
}