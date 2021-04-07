struct Data<'a> {
  data: [u8; 10],
  test_data: [&'a u8; 2],
}

impl<'a> Data<'a> {
  fn new() -> Data<'a> {
    Data {
      data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
      test_data: [&0, &0],
    }
  }

  fn run(&'a mut self) -> &[u8] {
    let data: &[u8] = &self.data[0..2];
    data
  }
}


fn main() {
  let mut data = Data::new();
  let t: &[u8] = data.run();
  println!("{:?}", t);
}