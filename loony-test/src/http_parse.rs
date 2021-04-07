use std::io::{Read, Write};
use std::fs::File;

trait DisplayArrayAsString {
  fn log(self);
}

impl DisplayArrayAsString for [u8; 1024] {
  fn log(self) {
    println!("{:?}", String::from_utf8_lossy(&self));
  }
}

impl DisplayArrayAsString for &[u8] {
  fn log(self) {
    println!("{:?}", String::from_utf8_lossy(&self));
  }
}

struct FileReader {}

impl FileReader {
  fn run(buffer: &mut [u8]) {
    let file = File::open("./buffer");
    match file {
      Ok(mut file) => {
        file.read(buffer).unwrap();
      }
      Err(_) => {}
    }
  }
}
fn main() {
  let mut buffer = [0; 1024];
  FileReader::run(&mut buffer);
}