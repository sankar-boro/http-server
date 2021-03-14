trait Describe {
    type Request;
    fn describe(&self) -> String;
}

impl<T> Describe for T where T: Fn() -> String {
    type Request = String;
    fn describe(&self) -> String {
        "Hello".to_string()
    }
}

type NewService = Box<
    dyn Describe<
        Request = String,
    >,
>;
struct Test {
    test: NewService
}

fn index() -> String {
    "Sankar".to_owned()
}

fn main() {
    Test {
        test: Box::new(index),
    };
}