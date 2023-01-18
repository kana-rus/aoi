mod body;
mod status;

use std::borrow::Cow;
use async_std::{net::TcpStream, io::WriteExt};

pub use body::{Body, Content};
pub use status::Status;
use crate::{header::HeaderKey, response::body::MediaType, time::now_fmt};

pub struct Response {
    pub(crate) additional_headers: String,
    pub status: Status,
    pub body:   Option<Body>,
} impl Response {
    pub fn error_context<Msg: Content>(mut self, msg: Msg) -> Self {
        match self.status {
            Status::OK | Status::Created | Status::NoContent => unreachable!(),
            _ => match self.body {
                None => self.body = Some(Body::text(msg)),
                Some(ref mut body) => match body.content_type() {
                    MediaType::text_plain => match body.content {
                        Cow::Owned(ref mut string) => match msg.into_content() {
                            Cow::Borrowed(msg_str) => *string += msg_str,
                            Cow::Owned(msg_string) => *string += &msg_string,
                        },
                        Cow::Borrowed(str) => body.content = Cow::Owned(match msg.into_content() {
                            Cow::Borrowed(msg_str) => msg_str.to_owned() + ": " + str,
                            Cow::Owned(msg_string) => msg_string + ": " + str,
                        }),
                    },
                    _ => unreachable!()
                },
            }
        }
        self
    }

    /// Add response header of `{key}: {value}`. key: &'static str | Header
    pub fn add_header<Key: HeaderKey>(&mut self, key: Key, value: &'static str) {
        self.additional_headers += key.as_key_str();
        self.additional_headers += ": ";
        self.additional_headers += value;
        self.additional_headers += "\r\n"
    }

    pub async fn write_to_stream(self, stream: &mut TcpStream) -> async_std::io::Result<()> {
        stream.write_all(match self.status {
            Status::NoContent => format!(
"HTTP/1.1 {}
Connection: Keep-Alive
Server: ohkami
Date: {}
Keep-Alive: timeout=5
{}
",
                self.status.as_response_status(),
                now_fmt(),
                self.additional_headers,
            ),
            _ => match self.body {
                None => format!(
"HTTP/1.1 {}
Connection: Keep-Alive
Content-Length: 0
Server: ohkami
Date: {}
Keep-Alive: timeout=5
{}
",
                    self.status.as_response_status(),
                    now_fmt(),
                    self.additional_headers,
                ),
                Some(body) => format!(
"HTTP/1.1 {}
Connection: Keep-Alive
Content-Type: {}
Content-Length: {}
Server: ohkami
Date: {}
Keep-Alive: timeout=5
{}
{}",
                    self.status.as_response_status(),
                    body.content_type().as_response_content_type(),
                    body.content_length(),
                    now_fmt(),
                    self.additional_headers,
                    body.content
                ),
            }
        }.as_bytes()).await
    }
}