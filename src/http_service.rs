use loony_service::{IntoServiceFactory, ServiceFactory};

struct HttpService {

}

impl HttpService {
    fn build() -> Self {
        HttpService {}
    }

    fn finish<T, F>(self, factory: T) -> Self 
    where T: IntoServiceFactory<F>,
    F: ServiceFactory<Request = (), Config = ()>
    {
        let fac = factory.into_factory();
        let ser = fac.new_service(());

        self
    }
}