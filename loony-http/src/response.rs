use std::convert::From;
pub struct Response {
    pub body: String,
    pub error: Option<String>,
}

// impl Response {
//     pub fn new(body: String) -> Self {

//         Self { body, error: None }
//     }
// }
impl From<String> for Response {
    fn from(body: String) -> Self {
        Self{
            body,
            error: None,
        }
    }
}

impl From<u8> for Response {
    fn from(body: u8) -> Self {
        Self{
            body: body.to_string(),
            error: None,
        }
    }
}
