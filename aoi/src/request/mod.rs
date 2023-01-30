use aoi_components::{
    json::JSON,
    request::{
        buffer::Buffer,
        range::{RangeMap, HeaderRangeMap}, Query,
    }, response::{Status, Response, Body}, result::ElseResponse,
};

use crate::result::Result;

pub struct Request<RequestBody: for <'j> JSON<'j>> {
    buffer:  Buffer,
    queries: RangeMap,
    headers: HeaderRangeMap,
    body:    RequestBody,
} impl<RequestBody: for <'j> JSON<'j>> Request<RequestBody> {
    pub fn query<'q, Q: Query<'q>>(&'q self, key: &str) -> Result<Q> {
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
}