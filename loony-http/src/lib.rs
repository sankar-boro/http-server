mod error;
mod response;

pub use error::Error;
pub use response::Response;
pub struct Http {}
pub fn http() {
    println!("Hello http");
}
