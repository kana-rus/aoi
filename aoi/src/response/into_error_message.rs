use std::borrow::Cow;

use aoi_components::response::Body;

pub trait ErrorMessage {
    fn as_error_message(self) -> Option<Body>;
}
impl ErrorMessage for Option<&'static str> {
    fn as_error_message(self) -> Option<Body> {
        self.map(|str| Cow::Borrowed(str))
    }
}
impl ErrorMessage for &'static str {
    fn as_error_message(self) -> Option<Body> {
        Some(Body::text(self))
    }
}
impl ErrorMessage for String {
    fn as_error_message(self) -> Option<Body> {
        Some(Body::text(self))
    }
}
impl ErrorMessage for &String {
    fn as_error_message(self) -> Option<Body> {
        Some(Body::text(self))
    }
}
