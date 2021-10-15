use crate::DB;
use crate::app;

pub async fn default(_: app::Data<DB>) -> String {
  String::from("Hello World!")
}