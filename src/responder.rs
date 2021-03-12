use loony_http::{Response};

pub trait Responder {
    fn respond(&self) -> String;
}

impl Responder for Response {
    fn respond(&self) -> String {
        match self {
            Response::Ok(ok) => {
                match ok {
                    Some(some_data) => some_data.clone(), 
                    None => "".to_string()
                }
            }
            Response::Err(err) => {
                match err {
                    Some(some_err_data) => some_err_data.clone(), 
                    None => "err".to_string()
                }
            }
        }
    }
}
