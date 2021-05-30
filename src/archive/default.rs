use crate::DB;
use crate::web;

pub async fn default(_: web::Data<DB>) -> String {
  String::from("Hello World!")
}