use crate::{config::AppService, request::HttpRequest};

pub trait HttpServiceFactory {
    fn register(self,  config: &mut AppService);
}
pub trait AppServiceFactory {
    fn register(&mut self, config: &mut AppService);
}
#[derive(Clone)]
pub struct HttpResponse {
    pub value: String,
}
#[derive(Clone)]
pub struct ServiceRequest(pub HttpRequest);
#[derive(Clone)]
pub struct ServiceResponse(pub HttpResponse);
pub(crate) struct ServiceFactoryWrapper<T> {
    factory: Option<T>,
}

impl<T> ServiceFactoryWrapper<T> {
    pub fn new(factory: T) -> Self {
        Self {
            factory: Some(factory),
        }
    }
}

impl<T> AppServiceFactory for ServiceFactoryWrapper<T>
where
    T: HttpServiceFactory,
{
    fn register(&mut self, config: &mut AppService) {
        if let Some(item) = self.factory.take() {
            item.register(config)
        }
    }
}