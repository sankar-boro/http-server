#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Header<'a> {
    /// The name portion of a header.
    ///
    /// A header name must be valid ASCII-US, so it's safe to store as a `&str`.
    pub name: &'a str,
    /// The value portion of a header.
    ///
    /// While headers **should** be ASCII-US, the specification allows for
    /// values that may not be, and so the value is stored as bytes.
    pub value: &'a [u8],
}

pub const EMPTY_HEADER: Header<'static> = Header { name: "", value: b"" };
pub struct Request<'a> {
    pub path: Option<&'a str>
}

impl<'a> Request<'a> {
    pub fn new(headers: &[Header; 16]) -> Self {
        Self {
            path: None
        }
    }

    pub fn parse(&mut self, buffer: &[u8]) {
        self.path = Some("/");
    }
}

#[derive(Clone)]
pub struct HttpRequest {
    pub url: String,
}