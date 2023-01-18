use crate::response::{Response, Status, Body};

pub trait ElseResponse {
    type Expect;
    fn _else<F: FnOnce() -> Response>(self, err: F) -> Result<Self::Expect, Response>;
}
impl<T> ElseResponse for Option<T> {
    type Expect = T;
    fn _else<F: FnOnce() -> Response>(self, err: F) -> Result<Self::Expect, Response> {
        self.ok_or_else(err)
    }
}
impl ElseResponse for bool {
    type Expect = ();
    fn _else<F: FnOnce() -> Response>(self, err: F) -> Result<Self::Expect, Response> {
        self.then_some(()).ok_or_else(err)
    }
}

pub trait ElseResponseWithErr<E> {
    type Expect;
    fn _else<F: FnOnce(E) -> Response>(self, err: F) -> Result<Self::Expect, Response>;
}
impl<T, E> ElseResponseWithErr<E> for std::result::Result<T, E> {
    type Expect = T;
    fn _else<F: FnOnce(E) -> Response>(self, err: F) -> Result<Self::Expect, Response> {
        self.map_err(err)
    }
}


impl From<std::io::Error> for Response {
    fn from(value: std::io::Error) -> Self {
        Self {
            additional_headers: String::new(),
            status: Status::InternalServerError,
            body:   Some(Body::text(value.to_string() + ": caused by I/O"))
        }
    }
}
impl From<serde_json::Error> for Response {
    fn from(value: serde_json::Error) -> Self {
        Self {
            additional_headers: String::new(),
            status: Status::InternalServerError,
            body:   Some(Body::text(value.to_string() + ": caused by json handling :: " + {
                if value.is_data() {
                    "invalid json data"
                } else if value.is_eof() {
                    "unexpected end of line"
                } else if value.is_io() {
                    "about io"
                } else { // value.is_syntax()
                    "wrong json syntax"
                }
            })),
        }
    }
}
