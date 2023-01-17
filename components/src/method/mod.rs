use crate::response::Response;

pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
} impl Method {
    pub fn parse(string: &str) -> Result<Self, Response> {
        
    }
}