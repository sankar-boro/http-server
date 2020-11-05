trait Model {
	type Response;
	fn get_name() -> Self::Response;
}

trait ModelFactory{
	type Response;
	fn new_model() -> Self::Response;
}

struct User;
impl ModelFactory for User{
	type Response = Self;
	fn new_model() -> Self::Response {
		User{}
	}
}

fn main() {
}

