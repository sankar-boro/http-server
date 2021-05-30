use crate::service::ServiceRequest;
pub trait Guard {
    /// Check if request matches predicate
    fn check(&self, request: &ServiceRequest) -> bool;
}

pub struct MethodGuard(http::Method);

/// Guard to match *GET* http method
pub fn Get() -> MethodGuard {
    MethodGuard(http::Method::GET)
}

/// Predicate to match *POST* http method
pub fn Post() -> MethodGuard {
    MethodGuard(http::Method::POST)
}

/// Predicate to match *PUT* http method
pub fn Put() -> MethodGuard {
    MethodGuard(http::Method::PUT)
}

/// Predicate to match *DELETE* http method
pub fn Delete() -> MethodGuard {
    MethodGuard(http::Method::DELETE)
}

/// Predicate to match *HEAD* http method
pub fn Head() -> MethodGuard {
    MethodGuard(http::Method::HEAD)
}

/// Predicate to match *OPTIONS* http method
pub fn Options() -> MethodGuard {
    MethodGuard(http::Method::OPTIONS)
}

/// Predicate to match *CONNECT* http method
pub fn Connect() -> MethodGuard {
    MethodGuard(http::Method::CONNECT)
}

/// Predicate to match *PATCH* http method
pub fn Patch() -> MethodGuard {
    MethodGuard(http::Method::PATCH)
}

/// Predicate to match *TRACE* http method
pub fn Trace() -> MethodGuard {
    MethodGuard(http::Method::TRACE)
}

/// Predicate to match specified http method
pub fn Method(method: http::Method) -> MethodGuard {
    MethodGuard(method)
}