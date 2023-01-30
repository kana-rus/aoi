pub mod buffer;
pub mod range;
pub mod parse;

use crate::{
    json::JSON,
    response::{Response, Status, Body},
    header::HeaderKey,
    result::{ElseResponseWithErr, ElseResponse},
};
use self::{
    buffer::Buffer,
    range::{RangeMap, HeaderRangeMap},
};

pub struct Request<J: for <'j> JSON<'j>> {
    pub buffer:  Buffer,

    pub queries: RangeMap,
    pub headers: HeaderRangeMap,
    pub body:    Option<J>,
} impl<J: for <'j> JSON<'j>> Request<J> {
    pub fn query<'req, Q: Query<'req>>(&'req self, key: &str) -> Result<Q, Response> {
        Query::parse(
            self.queries
                .read_match_part_of_buffer(key, &self.buffer)
                ._else(|| Response {
                    status: Status::BadRequest,
                    additional_headers: String::new(),
                    body: Some(Body::text(format!("expected query param `{key}`"))),
                })?
        )
    }
    /// Get value of the request header if it exists. key: &'static str | Header
    pub fn header<K: HeaderKey>(&self, key: K) -> Result<&str, Response> {
        let key_str = key.as_key_str();
        self.headers.get(key_str, &self.buffer)
            ._else(|| Response {
                status: Status::BadRequest,
                additional_headers: String::new(),
                body: Some(Body::text(format!("Header `{}` was not found", key_str))),
            })
    }
}

pub trait Query<'q> {fn parse(q: &'q str) -> Result<Self, Response> where Self: Sized;}
impl<'q> Query<'q> for &'q str {
    fn parse(q: &'q str) -> Result<Self, Response> {Ok(q)}
}
macro_rules! impl_for_int {
    ( $($int:ty)* ) => {
        impl<'q> Query<'q> for u8 {
            fn parse(q: &'q str) -> Result<Self, Response> {
                q.parse()
                    ._else(|_| Response {
                        status: Status::BadRequest,
                        additional_headers: String::new(),
                        body: Some(Body::text("format of query parameter is wrong")),
                    })
            }
        }
    };
} impl_for_int!(u8 u64 i64 i32 usize isize);
