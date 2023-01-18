pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
} impl Method {
    pub fn parse(string: &str) -> Self {
        match string {
            "GET"    => Self::GET,
            "POST"   => Self::POST,
            "PATCH"  => Self::PATCH,
            "DELETE" => Self::DELETE,
            other => panic!("Invalid request method: {other}")
        }
    }
}