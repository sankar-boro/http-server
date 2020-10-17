// #[derive(Debug)]
pub struct Route<'route, 'scope> {
    name: Vec<(&'route str, Box<dyn Fn() + 'static>)>,
    scope: &'scope str,
}

impl<'route, 'scope> Route<'route, 'scope> {
    pub fn route<T>(mut self, route: (&'route str, T)) -> Self where T: Fn() + 'static {
        self.name.push((route.0, Box::new(route.1)));
        self
    }

}

pub fn scope(scope: &str) -> Route {
    Route{
        scope,
        name:Vec::new(),
    }
}

pub fn get<T>(route: &str, get:T) -> (&str, T) where T: Fn() {
    (route, get)
}