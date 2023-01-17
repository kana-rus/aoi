mod body;

use crate::status::Status;
use;

pub struct Response {
    pub(crate) additional_headers: String,
    pub status: Status,
    pub body:   Option<Body>,
}