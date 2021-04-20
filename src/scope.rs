use crate::{
    resource::{
        Resource, 
        ResourceService, 
    },
    route::Route};
use loony_service::{ServiceFactory};

type ScopeFactory = Box<
    dyn ServiceFactory<
        Request = String, 
        Response = String, 
        Error = (), 
        Service=ResourceService
    >
>;

pub struct Scope {
    scope: String,
    pub services: Vec<ScopeFactory>,
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
        }
    }

    pub fn route(mut self, path: &str, route: Route) -> Self {
        let resource = Resource::new(path, &self.scope).route(route);
        self.services.push(Box::new(resource));
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use loony_service::Service;

    async fn index(req: String) -> String {
        req
    }

    #[test]
    fn scope() {
        let scope = Scope::new("/user");
        let route = scope
        .route("/get", Route::new().route(index))
        .route("/delete", Route::new().route(index));
        let services = route.services.iter();
        for service in services {
            let ser = service.new_service();
            let mut route = ser.route;
            let s = route.call("name".to_string());
            assert_eq!("name", s);
        }
    }
}