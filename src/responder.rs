pub trait Responder {
    fn respond(self) -> String;
}

impl Responder for String {
    fn respond(self) -> String {
        self
    }
}
