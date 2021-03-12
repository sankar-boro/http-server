use std::convert::From;

pub enum Response {
    Ok(Option<String>),
    Err(Option<String>),
}

// impl Response {
//     pub fn new(body: String) -> Self {

//         Self { body, error: None }
//     }
// }

impl Response {
    pub fn ok(val: String) -> Response {
        Response::Ok(Some(val))
    }
    
    pub fn err(val: String) -> Response {
        Response::Err(Some(val))
    }
}

// impl From<String> for Response {
//     fn from(body: String) -> Self {
//         Self{
//             body,
//             error: None,
//         }
//     }
// }

// impl From<u8> for Response {
//     fn from(body: u8) -> Self {
//         Self{
//             body: body.to_string(),
//             error: None,
//         }
//     }
// }
