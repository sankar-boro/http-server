use crate::DB;
pub async fn default(_: DB) -> String {
  String::from("Hello World!")
}