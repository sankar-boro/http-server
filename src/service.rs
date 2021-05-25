use crate::request::HttpRequest;
#[derive(Clone)]
pub struct HttpResponse {
    pub value: String,
}
#[derive(Clone)]
pub struct ServiceRequest(pub HttpRequest);
#[derive(Clone)]
pub struct ServiceResponse(pub HttpResponse);