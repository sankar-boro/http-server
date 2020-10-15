use loony_http::{Error, Response};
pub trait Responder {
    // type Error: Into<Error>;
    // fn respond(body: String) -> Result<Response, Error>;
    // fn respond(&self) -> String;
}

impl Responder for Response {
    // fn respond(body: String) -> Result<Response, Error> {
    // fn respond(&self) -> String {
    //     // Ok(Response { body, error: None })
    //     self.body.to_string()
    // }
}
