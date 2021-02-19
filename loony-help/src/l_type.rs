trait Factory {
    type NewFactory;
}
type Something = Box<Factory<NewFactory =String>>;

trait GetName {
    fn get_name() -> String;
}
struct User<N: GetName> {
    name: N,
}

impl<N: GetName> User<N> {
    fn new(name:N) -> Self {
        Self {
            name,
        }
    }
}