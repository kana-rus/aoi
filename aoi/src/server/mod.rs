use aoi_components::response::{Response, Content, Status};

use crate::{response::{IntoOK, ErrorMessage}, result::HandleResult};

pub trait Server {
    fn OK<B: IntoOK<From>, From>(&self, body: B) -> HandleResult {
        Ok(Self {
            additional_headers: String::new(),
            status:             Status::OK,
            body:               body.into_ok()?,
        })
    }
    fn Created<Content: IntoCreated<From>, From>(&self, content: Content) -> HandleResult {
        Ok(Self {
            additional_headers: String::new(),
            status:             Status::Created,
            body:               Some(body.into_created()?),
        })
    }
    fn NoContent(&self, ) -> HandleResult {
        Ok(Self {
            additional_headers: String::new(),
            status:             Status::Created,
            body:               None,
        })
    }

    fn BadRequest<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::BadRequest,
            body:   message.as_error_message(),
        }
    }
    fn Unauthorized<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::Unauthorized,
            body:   message.as_error_message(),
        }
    }
    fn Forbidden<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::Forbidden,
            body:   message.as_error_message(),
        }
    }
    fn NotFound<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::NotFound,
            body:   message.as_error_message(),
        }
    }
    fn InternalServerError<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::InternalServerError,
            body:   message.as_error_message(),
        }
    }
    fn NotImplemented<Message: ErrorMessage>(&self, message: Message) -> Response {
        Response {
            additional_headers: String::new(),
            status: Status::NotImplemented,
            body:   message.as_error_message(),
        }
    }
}
