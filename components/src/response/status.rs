pub enum Status {
    OK,
    Created,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
} impl Status {
    pub(crate) fn as_response_status(&self) -> &'static str {
        match self {
            Self::OK                  => "200 OK",
            Self::Created             => "201 Created",
            Self::NoContent           => "204 NoContent",
            Self::BadRequest          => "400 BadRequest",
            Self::Unauthorized        => "401 Unauthorized",
            Self::Forbidden           => "403 Forbidden",
            Self::NotFound            => "404 NotFound",
            Self::InternalServerError => "500 InternalServerError",
            Self::NotImplemented      => "501 NotImplemented",
        }
    }
}

