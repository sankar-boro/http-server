use crate::Request;
pub trait FromRequest {

}

impl FromRequest for String {

}

impl FromRequest for Request {}