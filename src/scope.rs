use crate::route::Route;
pub struct Scope{
    scope: String,
    services: Vec<(String, Route)>
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
        }
    }

    pub fn route(mut self, path: &str, route: Route) -> Self {
        self.services.push((path.to_owned(), route));
        self
    }
}