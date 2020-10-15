#[derive(Debug)]
pub struct Route<'route, 'scope> {
    name: Vec<&'route str>,
    scope: &'scope str,
}

impl<'route, 'scope> Route<'route, 'scope> {
    pub fn route(mut self, route: &'route str) -> Self {
        self.name.push(route);
        self
    }

}

pub fn scope(scope: &str) -> Route {
    Route{
        scope,
        name:Vec::new(),
    }
}

pub fn get(route: &str) -> &str {
    route
}