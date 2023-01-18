mod body;
mod status;

pub use body::Body;
pub use status::Status;

pub struct Response {
    pub(crate) additional_headers: String,
    pub status: Status,
    pub body:   Option<Body>,
} impl Response {
    
}